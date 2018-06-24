use ::measurement::Measurement;

pub mod line;

/// `Measurement` serializer.
pub trait Serializer: Send + Sync {
    /// Serializes measurement to String.
    fn serialize(&self, measurement: &Measurement) -> String;
}
