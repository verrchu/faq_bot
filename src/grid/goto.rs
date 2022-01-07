use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

use crate::utils;

static LANG: &str = "ru";

pub async fn goto(hash: String) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let buttons = InlineKeyboardMarkup::new(navigation("test"));

    Ok(("test".to_string(), buttons))
}

fn navigation(back: &str) -> Vec<Vec<InlineKeyboardButton>> {
    vec![vec![
        InlineKeyboardButton::new(
            "<<".to_string(),
            InlineKeyboardButtonKind::CallbackData(cb_data("/")),
        ),
        InlineKeyboardButton::new(
            "<".to_string(),
            InlineKeyboardButtonKind::CallbackData(cb_data(back)),
        ),
    ]]
}

fn cb_data(goto: &str) -> String {
    format!("/goto#{}", utils::hash(goto))
}