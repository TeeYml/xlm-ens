#[cfg(test)]
mod tests {
    use crate::types::RegistryEntry;
    use crate::RegistryContract;
    use xlm_ns_common::NameRecord;

    #[test]
    fn registers_and_transfers_name() {
        let mut registry = RegistryContract::default();
        let now_unix = 100;
        let entry = RegistryEntry::new(
            NameRecord::new("timmy", "GABC", Some("GABC".into()), now_unix, 1_000, 2_000),
            None,
            now_unix,
        );

        registry.register(entry, now_unix).unwrap();
        registry.transfer("timmy.xlm", "GABC", "GDEF", now_unix + 1).unwrap();

        let stored = registry.resolve("timmy.xlm", now_unix + 1).unwrap();
        assert_eq!(stored.record.owner, "GDEF");
    }
}
