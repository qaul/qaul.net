use bluer::Address;

pub fn mac_to_string(addr: &Address) -> String {
    addr.map(|octet| format!("{:02x?}", octet)).join(":")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_prints_address() {
        assert_eq!(
            mac_to_string(&Address::any()),
            String::from("00:00:00:00:00:00")
        )
    }
}
