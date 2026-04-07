#[cfg(test)]
mod tests {
    use crate::{build_gmp_message, target_for_chain, BridgeContract};

    #[test]
    fn builds_axelar_payloads() {
        let payload = build_gmp_message("timmy.xlm", "base", "0xbaseResolver");
        assert!(payload.contains("timmy.xlm"));
        let target = target_for_chain("base").unwrap();
        assert_eq!(target.resolver, "0xbaseResolver");
    }

    #[test]
    fn registers_supported_routes() {
        let mut bridge = BridgeContract::default();
        bridge.register_chain("base").unwrap();

        let route = bridge.route("base").unwrap();
        assert_eq!(route.destination_resolver, "0xbaseResolver");
        assert!(bridge.build_message("timmy.xlm", "base").is_ok());
    }
}
