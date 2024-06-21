use bluer::Uuid;

pub fn main_service_uuid() -> Uuid {
    Uuid::parse_str("99e91399-80ed-4943-9bcb-39c532a76023").unwrap()
}
pub fn msg_service_uuid() -> Uuid {
    Uuid::parse_str("99e91400-80ed-4943-9bcb-39c532a76023").unwrap()
}
pub fn read_char() -> Uuid {
    Uuid::parse_str("99e91401-80ed-4943-9bcb-39c532a76023").unwrap()
}
pub fn msg_char() -> Uuid {
    Uuid::parse_str("99e91402-80ed-4943-9bcb-39c532a76023").unwrap()
}
pub fn gd_char() -> Uuid {
    Uuid::parse_str("99e91403-80ed-4943-9bcb-39c532a76023").unwrap()
}
