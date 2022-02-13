use crate::{
    grid::callback,
    l10n::{buttons, messages, Lang},
};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub fn render(lang: Lang) -> (String, InlineKeyboardMarkup) {
    let header = messages::greeting(lang).to_string();
    let buttons = vec![vec![InlineKeyboardButton::new(
        buttons::begin(lang),
        InlineKeyboardButtonKind::CallbackData(callback::data(callback::Command::Goto, "/")),
    )]];

    (header, InlineKeyboardMarkup::new(buttons))
}
