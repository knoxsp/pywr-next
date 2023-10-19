use crate::aggregated_node::AggregatedNodeIndex;
use crate::aggregated_storage_node::AggregatedStorageNodeIndex;
use crate::edge::EdgeIndex;
use crate::network::Network;
use crate::node::NodeIndex;
use crate::parameters::{MultiValueParameterIndex, ParameterIndex};
use crate::state::State;
use crate::virtual_storage::VirtualStorageIndex;
use crate::PywrError;

#[derive(Clone, Debug, PartialEq)]
pub struct VolumeBetweenControlCurves {
    max_volume: Box<Metric>,
    upper: Option<Box<Metric>>,
    lower: Option<Box<Metric>>,
}

impl VolumeBetweenControlCurves {
    pub fn new(max_volume: Metric, upper: Option<Metric>, lower: Option<Metric>) -> Self {
        Self {
            max_volume: Box::new(max_volume),
            upper: upper.map(Box::new),
            lower: lower.map(Box::new),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metric {
    NodeInFlow(NodeIndex),
    NodeOutFlow(NodeIndex),
    NodeVolume(NodeIndex),
    NodeInFlowDeficit(NodeIndex),
    NodeProportionalVolume(NodeIndex),
    AggregatedNodeInFlow(AggregatedNodeIndex),
    AggregatedNodeOutFlow(AggregatedNodeIndex),
    AggregatedNodeVolume(AggregatedStorageNodeIndex),
    AggregatedNodeProportionalVolume(AggregatedStorageNodeIndex),
    EdgeFlow(EdgeIndex),
    ParameterValue(ParameterIndex),
    MultiParameterValue((MultiValueParameterIndex, String)),
    VirtualStorageVolume(VirtualStorageIndex),
    VirtualStorageProportionalVolume(VirtualStorageIndex),
    VolumeBetweenControlCurves(VolumeBetweenControlCurves),
    MultiNodeInFlow {
        indices: Vec<NodeIndex>,
        name: String,
        sub_name: Option<String>,
    },
    // TODO implement other MultiNodeXXX variants
    Constant(f64),
}

impl Metric {
    pub fn get_value(&self, model: &Network, state: &State) -> Result<f64, PywrError> {
        match self {
            Metric::NodeInFlow(idx) => Ok(state.get_network_state().get_node_in_flow(idx)?),
            Metric::NodeOutFlow(idx) => Ok(state.get_network_state().get_node_out_flow(idx)?),
            Metric::NodeVolume(idx) => Ok(state.get_network_state().get_node_volume(idx)?),
            Metric::AggregatedNodeInFlow(idx) => {
                let node = model.get_aggregated_node(idx)?;
                // TODO this could be more efficient with an iterator method? I.e. avoid the `Vec<_>` allocation
                node.get_nodes()
                    .iter()
                    .map(|idx| state.get_network_state().get_node_in_flow(idx))
                    .sum::<Result<_, _>>()
            }
            Metric::AggregatedNodeOutFlow(idx) => {
                let node = model.get_aggregated_node(idx)?;
                // TODO this could be more efficient with an iterator method? I.e. avoid the `Vec<_>` allocation
                node.get_nodes()
                    .iter()
                    .map(|idx| state.get_network_state().get_node_out_flow(idx))
                    .sum::<Result<_, _>>()
            }
            Metric::NodeProportionalVolume(idx) => {
                let max_volume = model.get_node(idx)?.get_current_max_volume(model, state)?;
                Ok(state
                    .get_network_state()
                    .get_node_proportional_volume(idx, max_volume)?)
            }
            Metric::EdgeFlow(idx) => Ok(state.get_network_state().get_edge_flow(idx)?),
            Metric::ParameterValue(idx) => Ok(state.get_parameter_value(*idx)?),
            Metric::MultiParameterValue((idx, key)) => Ok(state.get_multi_parameter_value(*idx, key)?),
            Metric::VirtualStorageVolume(idx) => Ok(state.get_network_state().get_virtual_storage_volume(idx)?),
            Metric::VirtualStorageProportionalVolume(idx) => {
                let max_volume = model.get_virtual_storage_node(idx)?.get_max_volume(model, state)?;
                Ok(state
                    .get_network_state()
                    .get_virtual_storage_proportional_volume(idx, max_volume)?)
            }
            Metric::Constant(v) => Ok(*v),
            Metric::AggregatedNodeVolume(idx) => {
                let node = model.get_aggregated_storage_node(idx)?;
                node.nodes
                    .iter()
                    .map(|idx| state.get_network_state().get_node_volume(idx))
                    .sum::<Result<_, _>>()
            }
            Metric::AggregatedNodeProportionalVolume(idx) => {
                let node = model.get_aggregated_storage_node(idx)?;
                let volume: f64 = node
                    .nodes
                    .iter()
                    .map(|idx| state.get_network_state().get_node_volume(idx))
                    .sum::<Result<_, _>>()?;

                let max_volume: f64 = node
                    .nodes
                    .iter()
                    .map(|idx| model.get_node(idx)?.get_current_max_volume(model, state))
                    .sum::<Result<_, _>>()?;
                // TODO handle divide by zero
                Ok(volume / max_volume)
            }
            Metric::MultiNodeInFlow { indices, .. } => {
                let flow = indices
                    .iter()
                    .map(|idx| state.get_network_state().get_node_in_flow(idx))
                    .sum::<Result<_, _>>()?;
                Ok(flow)
            }
            Metric::NodeInFlowDeficit(idx) => {
                let node = model.get_node(idx)?;
                let flow = state.get_network_state().get_node_in_flow(idx)?;
                let max_flow = node.get_current_max_flow(model, state)?;
                Ok(max_flow - flow)
            }
            Metric::VolumeBetweenControlCurves(vol) => {
                let max_volume = vol.max_volume.get_value(model, state)?;
                let lower = vol
                    .lower
                    .as_ref()
                    .map_or(Ok(0.0), |metric| metric.get_value(model, state))?;
                let upper = vol
                    .upper
                    .as_ref()
                    .map_or(Ok(1.0), |metric| metric.get_value(model, state))?;

                // TODO handle invalid bounds
                Ok(max_volume * (upper - lower))
            }
        }
    }
}
