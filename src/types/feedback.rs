use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Feedback {
    #[builder(setter(into))]
    pub username: String,
    #[builder(setter(into))]
    pub text: String,
}

impl Feedback {
    pub fn as_pairs(&self) -> Vec<(&str, &str)> {
        vec![("username", &self.username), ("text", &self.text)]
    }
}
