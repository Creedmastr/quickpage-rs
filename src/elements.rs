pub fn element(name: &str, opt: &str, content: String) -> String {
    return format!("    <{0} {1}> {2} </{0}>\n    ", name, opt, content);
}