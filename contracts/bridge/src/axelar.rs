pub fn build_gmp_message(name: &str, destination_chain: &str, resolver: &str) -> String {
    format!(
        "{{\"name\":\"{name}\",\"destination_chain\":\"{destination_chain}\",\"resolver\":\"{resolver}\"}}"
    )
}
