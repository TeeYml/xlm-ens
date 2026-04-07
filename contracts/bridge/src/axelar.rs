pub fn build_gmp_message(name: &str, destination_chain: &str, resolver: &str) -> String {
    format!(
        concat!(
            "{{",
            "\"type\":\"xlm-ns-resolution\",",
            "\"name\":\"{name}\",",
            "\"destination_chain\":\"{destination_chain}\",",
            "\"resolver\":\"{resolver}\"",
            "}}"
        )
    )
}
