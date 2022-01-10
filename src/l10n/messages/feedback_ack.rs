use crate::l10n::Lang;

static RU: &str = r#"
Спасибо за Ваше мнение\.
Мы обязательно его учтем\.
"#;

pub fn feedback_ack(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => RU.trim(),
    }
}
