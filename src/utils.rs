pub fn hash(input: &str) -> String {
    format!("{:x}", md5::compute(input))
}
