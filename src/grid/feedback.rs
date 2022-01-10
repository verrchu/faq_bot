use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub fn add_feedback_button(markup: &mut InlineKeyboardMarkup) {
    markup.inline_keyboard.push(vec![InlineKeyboardButton::new(
        "FEEDBACK",
        InlineKeyboardButtonKind::CallbackData("/feedback".to_string()),
    )]);
}
