use crate::l10n::Lang;

static RU: &str = r#"
Пожалуйста, напишите Ваш отзыв в следующем сообщении\.
Постарайтесь уместить его в *одно* сообщение\.
"#;

pub fn feedback_prelude(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => RU.trim(),
    }
}
