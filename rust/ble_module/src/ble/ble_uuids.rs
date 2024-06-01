use bluer::Uuid;

pub fn main_service_uuid() -> Uuid {
    Uuid::parse_str("99E91399-80ED-4943-9BCB-39C532A76023").unwrap()
}
pub fn msg_service_uuid() -> Uuid {
    Uuid::parse_str("99E91400-80ED-4943-9BCB-39C532A76023").unwrap()
}
pub fn read_char() -> Uuid {
    Uuid::parse_str("99E91401-80ED-4943-9BCB-39C532A76023").unwrap()
}
pub fn msg_char() -> Uuid {
    Uuid::parse_str("99E91402-80ED-4943-9BCB-39C532A76023").unwrap()
}
pub fn gd_char() -> Uuid {
    Uuid::parse_str("99E91403-80ED-4943-9BCB-39C532A76023").unwrap()
}
