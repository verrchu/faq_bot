use crate::l10n::Lang;

static RU: &str = "[ Оставить отзыв ]";

pub fn feedback(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => RU,
    }
}
