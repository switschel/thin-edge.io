use tedge_mqtt_ext::MqttMessage;

use super::error::ConversionError;

#[derive(Debug)]
pub struct SizeThreshold(pub usize);

impl SizeThreshold {
    pub fn validate(&self, input: &MqttMessage) -> Result<(), ConversionError> {
        let actual_size = input.payload_bytes().len();
        let threshold = self.0;
        if actual_size > threshold {
            Err(ConversionError::SizeThresholdExceeded {
                topic: input.topic.name.clone(),
                actual_size,
                threshold,
            })
        } else {
            Ok(())
        }
    }
}
