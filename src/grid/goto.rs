use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn goto(hash: String) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let buttons = InlineKeyboardMarkup::new([[
        InlineKeyboardButton::new(
            "<<".to_string(),
            InlineKeyboardButtonKind::CallbackData("test".to_string()),
        ),
        InlineKeyboardButton::new(
            "<".to_string(),
            InlineKeyboardButtonKind::CallbackData("test".to_string()),
        ),
    ]]);

    Ok(("test".to_string(), buttons))
}
