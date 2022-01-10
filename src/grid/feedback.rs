use crate::l10n::{self, Lang};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub fn add_feedback_button(markup: &mut InlineKeyboardMarkup, lang: Lang) {
    markup.inline_keyboard.push(vec![InlineKeyboardButton::new(
        l10n::buttons::feedback(lang),
        InlineKeyboardButtonKind::CallbackData("/feedback".to_string()),
    )]);
}
