use crate::l10n::Lang;

static RU: &str = "Старт";

pub fn begin(lang: Lang) -> &'static str {
    match lang {
        Lang::Ru => RU,
    }
}
