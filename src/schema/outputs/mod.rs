mod csv;
mod hdf;

pub use self::csv::CsvOutput;
use crate::PywrError;
pub use hdf::Hdf5Output;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Output {
    CSV(CsvOutput),
    HDF5(Hdf5Output),
}

impl Output {
    pub fn add_to_model(
        &self,
        model: &mut crate::model::Model,
        schema: &crate::schema::PywrModel,
    ) -> Result<(), PywrError> {
        match self {
            Self::CSV(o) => o.add_to_model(model, schema),
            Self::HDF5(o) => o.add_to_model(model, schema),
        }
    }
}
