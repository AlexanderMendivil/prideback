mod services{

    pub fn guests() -> &'static str {
        "guests"
    }
}

pub fn get_services(){
    crate::services::services::guests();
}