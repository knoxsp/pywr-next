use crate::data_tables::LoadedTableCollection;
use crate::error::SchemaError;
use crate::nodes::NodeMeta;
use crate::parameters::DynamicFloatValue;
use num::Zero;
use pywr_core::aggregated_node::Factors;
use pywr_core::metric::Metric;
use std::path::Path;

#[doc = svgbobdoc::transform!(
/// A node used to represent a water treatment works (WTW) with optional losses.
///
/// This nodes comprises an internal structure that allows specifying a minimum and
/// maximum total net flow, an optional loss factor applied as a proportion of *net*
/// flow, and an optional "soft" minimum flow.
///
/// When a loss factor is not given the `loss` node is not created. When a non-zero loss
/// factor is provided `Output` and `Aggregated` nodes are created. The aggregated node
/// is given factors that require the flow through the output node to be equal to
/// loss factor mulitplied by the net flow. I.e. total gross flow will become
/// (1 + loss factor) * net flow.
///
///
/// ```svgbob
///                          <node>.net_soft_min_flow
///                           .--->L ---.
///            <node>.net    |           |     D
///          .------>L ------|           |--->*- - -
///      U  |                |           |
///     -*--|                 '--->L ---'
///         |                <node>.net_above_soft_min_flow
///          '------>O
///            <node>.loss
/// ```
///
)]
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct WaterTreatmentWorks {
    /// Node metadata
    #[serde(flatten)]
    pub meta: NodeMeta,
    /// The proportion of net flow that is lost to the loss node.
    pub loss_factor: Option<DynamicFloatValue>,
    /// The minimum flow through the `net` flow node.
    pub min_flow: Option<DynamicFloatValue>,
    /// The maximum flow through the `net` flow node.
    pub max_flow: Option<DynamicFloatValue>,
    /// The maximum flow applied to the `net_soft_min_flow` node which is typically
    /// used as a "soft" minimum flow.
    pub soft_min_flow: Option<DynamicFloatValue>,
    /// The cost applied to the `net_soft_min_flow` node.
    pub soft_min_flow_cost: Option<DynamicFloatValue>,
    /// The cost applied to the `net` flow node.
    pub cost: Option<DynamicFloatValue>,
}

impl WaterTreatmentWorks {
    fn loss_sub_name() -> Option<&'static str> {
        Some("loss")
    }

    fn net_sub_name() -> Option<&'static str> {
        Some("net")
    }
    fn agg_sub_name() -> Option<&'static str> {
        Some("agg")
    }

    fn net_soft_min_flow_sub_name() -> Option<&'static str> {
        Some("net_soft_min_flow")
    }

    fn net_above_soft_min_flow_sub_name() -> Option<&'static str> {
        Some("net_above_soft_min_flow")
    }

    pub fn add_to_model(&self, model: &mut pywr_core::model::Model) -> Result<(), SchemaError> {
        let idx_net = model.add_link_node(self.meta.name.as_str(), Self::net_sub_name())?;
        let idx_soft_min_flow = model.add_link_node(self.meta.name.as_str(), Self::net_soft_min_flow_sub_name())?;
        let idx_above_soft_min_flow =
            model.add_link_node(self.meta.name.as_str(), Self::net_above_soft_min_flow_sub_name())?;

        // Create the internal connections
        model.connect_nodes(idx_net, idx_soft_min_flow)?;
        model.connect_nodes(idx_net, idx_above_soft_min_flow)?;

        if self.loss_factor.is_some() {
            let idx_loss = model.add_output_node(self.meta.name.as_str(), Self::loss_sub_name())?;
            // This aggregated node will contain the factors to enforce the loss
            model.add_aggregated_node(
                self.meta.name.as_str(),
                Self::agg_sub_name(),
                &[idx_net, idx_loss],
                None,
            )?;
        }

        Ok(())
    }

    pub fn set_constraints(
        &self,
        model: &mut pywr_core::model::Model,
        tables: &LoadedTableCollection,
        data_path: Option<&Path>,
    ) -> Result<(), SchemaError> {
        if let Some(cost) = &self.cost {
            let value = cost.load(model, tables, data_path)?;
            model.set_node_cost(self.meta.name.as_str(), Self::net_sub_name(), value.into())?;
        }

        if let Some(max_flow) = &self.max_flow {
            let value = max_flow.load(model, tables, data_path)?;
            model.set_node_max_flow(self.meta.name.as_str(), Self::net_sub_name(), value.into())?;
        }

        if let Some(min_flow) = &self.min_flow {
            let value = min_flow.load(model, tables, data_path)?;
            model.set_node_min_flow(self.meta.name.as_str(), Self::net_sub_name(), value.into())?;
        }

        // soft min flow constraints; This typically applies a negative cost upto a maximum
        // defined by the `soft_min_flow`
        if let Some(cost) = &self.soft_min_flow_cost {
            let value = cost.load(model, tables, data_path)?;
            model.set_node_cost(
                self.meta.name.as_str(),
                Self::net_soft_min_flow_sub_name(),
                value.into(),
            )?;
        }
        if let Some(min_flow) = &self.soft_min_flow {
            let value = min_flow.load(model, tables, data_path)?;
            model.set_node_max_flow(
                self.meta.name.as_str(),
                Self::net_soft_min_flow_sub_name(),
                value.into(),
            )?;
        }

        if let Some(loss_factor) = &self.loss_factor {
            // Handle the case where we a given a zero loss factor
            // The aggregated node does not support zero loss factors so filter them here.
            let lf = match loss_factor.load(model, tables, data_path)? {
                Metric::Constant(f) => {
                    if f.is_zero() {
                        None
                    } else {
                        Some(Metric::Constant(f))
                    }
                }
                m => Some(m),
            };

            if let Some(lf) = lf {
                // Set the factors for the loss
                // TODO allow for configuring as proportion of gross.
                let factors = Factors::Ratio(vec![Metric::Constant(1.0), lf]);
                model.set_aggregated_node_factors(self.meta.name.as_str(), Self::agg_sub_name(), Some(factors))?;
            }
        }

        Ok(())
    }

    pub fn input_connectors(&self) -> Vec<(&str, Option<String>)> {
        // Connect directly to the total net
        let mut connectors = vec![(self.meta.name.as_str(), Self::net_sub_name().map(|s| s.to_string()))];
        // Only connect to the loss link if it is created
        if self.loss_factor.is_some() {
            connectors.push((self.meta.name.as_str(), Self::loss_sub_name().map(|s| s.to_string())))
        }
        connectors
    }

    pub fn output_connectors(&self) -> Vec<(&str, Option<String>)> {
        // Connect to the split of the net flow.
        vec![
            (
                self.meta.name.as_str(),
                Self::net_soft_min_flow_sub_name().map(|s| s.to_string()),
            ),
            (
                self.meta.name.as_str(),
                Self::net_above_soft_min_flow_sub_name().map(|s| s.to_string()),
            ),
        ]
    }

    pub fn default_metric(&self, model: &pywr_core::model::Model) -> Result<Metric, SchemaError> {
        let idx = model.get_node_index_by_name(self.meta.name.as_str(), Self::net_sub_name().as_deref())?;
        Ok(Metric::NodeOutFlow(idx))
    }
}

#[cfg(test)]
mod tests {
    use crate::model::PywrModel;
    use crate::nodes::WaterTreatmentWorks;
    use ndarray::Array2;
    use pywr_core::metric::Metric;
    use pywr_core::recorders::AssertionRecorder;
    use pywr_core::test_utils::run_all_solvers;

    #[test]
    fn test_wtw_schema_load() {
        let data = r#"
                {
                  "type": "WaterTreatmentWorks",
                  "name": "My WTW",
                  "comment": null,
                  "position": null,
                  "loss_factor": {
                    "index": "My WTW",
                    "table": "loss_factors"
                  },
                  "soft_min_flow": 105,
                  "cost": 2.29,
                  "max_flow": {
                    "type": "InlineParameter",
                    "definition": {
                        "type": "ControlCurve",
                        "name": "My WTW max flow",
                        "control_curves": [
                          {
                            "type": "Parameter",
                            "name": "A control curve"
                          }
                        ],
                        "values": [
                          {
                            "type": "Parameter",
                            "name": "a max flow"
                          },
                          0.0
                        ],
                        "storage_node": "My reservoir"
                    }
                  },
                  "soft_min_flow_cost": {
                    "type": "Parameter",
                    "name": "my_min_flow_cost"
                  }
                }
            "#;

        let node: WaterTreatmentWorks = serde_json::from_str(data).unwrap();

        assert_eq!(node.meta.name, "My WTW");
    }

    fn model_str() -> &'static str {
        r#"
            {
                "metadata": {
                    "title": "WTW Test 1",
                    "description": "Test WTW work",
                    "minimum_version": "0.1"
                },
                "timestepper": {
                    "start": "2015-01-01",
                    "end": "2015-12-31",
                    "timestep": 1
                },
                "nodes": [
                    {
                        "name": "input1",
                        "type": "Input",
                        "flow": 15
                    },
                    {
                        "name": "wtw1",
                        "type": "WaterTreatmentWorks",
                        "max_flow": 10.0,
                        "loss_factor": 0.1
                    },
                    {
                        "name": "demand1",
                        "type": "Output",
                        "max_flow": 15.0,
                        "cost": -10
                    }
                ],
                "edges": [
                    {
                        "from_node": "input1",
                        "to_node": "wtw1"
                    },
                    {
                        "from_node": "wtw1",
                        "to_node": "demand1"
                    }
                ]
            }
            "#
    }

    #[test]
    fn test_model_schema() {
        let data = model_str();
        let schema: PywrModel = serde_json::from_str(data).unwrap();

        assert_eq!(schema.nodes.len(), 3);
        assert_eq!(schema.edges.len(), 2);
    }

    #[test]
    fn test_model_run() {
        let data = model_str();
        let schema: PywrModel = serde_json::from_str(data).unwrap();
        let (mut model, timestepper) = schema.build_model(None, None).unwrap();

        assert_eq!(model.nodes.len(), 6);
        assert_eq!(model.edges.len(), 6);

        let scenario_indices = model.get_scenario_indices();

        // Setup expected results
        // Set-up assertion for "input" node
        // TODO write some helper functions for adding these assertion recorders
        let idx = model.get_node_by_name("input1", None).unwrap().index();
        let expected = Array2::from_elem((timestepper.timesteps().len(), scenario_indices.len()), 11.0);
        let recorder = AssertionRecorder::new("input-flow", Metric::NodeOutFlow(idx), expected, None, None);
        model.add_recorder(Box::new(recorder)).unwrap();

        let idx = model.get_node_by_name("demand1", None).unwrap().index();
        let expected = Array2::from_elem((timestepper.timesteps().len(), scenario_indices.len()), 10.0);
        let recorder = AssertionRecorder::new("demand-flow", Metric::NodeInFlow(idx), expected, None, None);
        model.add_recorder(Box::new(recorder)).unwrap();

        // Test all solvers
        run_all_solvers(&model, &timestepper);
    }
}