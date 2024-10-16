pub const MY_RUST_SERVICE_SERVER: &str = "my_rust_service_server";

pub fn get_services_dependence_list() -> Vec<String> {
    let mut services_dependence_list = Vec::new();
    services_dependence_list.push(MY_RUST_SERVICE_SERVER.to_string());    
    services_dependence_list
}   