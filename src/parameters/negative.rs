use crate::metric::Metric;
use crate::parameters::{Parameter, ParameterMeta};
use crate::scenario::ScenarioIndex;
use crate::state::State;
use crate::timestep::Timestep;
use crate::PywrError;
use std::any::Any;

pub struct NegativeParameter {
    meta: ParameterMeta,
    metric: Metric,
}

impl NegativeParameter {
    pub fn new(name: &str, metric: Metric) -> Self {
        Self {
            meta: ParameterMeta::new(name),
            metric,
        }
    }
}

impl Parameter for NegativeParameter {
    fn meta(&self) -> &ParameterMeta {
        &self.meta
    }
    fn compute(
        &self,
        _timestep: &Timestep,
        _scenario_index: &ScenarioIndex,
        state: &State,
        _internal_state: &mut Option<Box<dyn Any>>,
    ) -> Result<f64, PywrError> {
        // Current value
        let x = self.metric.get_value(state)?;
        Ok(-x)
    }
}
