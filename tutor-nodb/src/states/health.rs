use std::sync::Mutex;

#[derive(Debug)]
pub struct HealthState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
}
impl HealthState {
    pub fn new(health_check_message: &str) -> Self {
        Self {
            health_check_response: health_check_message.into(),
            visit_count: Default::default(),
        }
    }
}
