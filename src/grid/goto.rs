use std::path::Path;

use crate::{utils, Db};

use redis::Commands;
use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

static LANG: &str = "ru";

pub async fn goto(hash: String, mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key = db.get_key(&hash).await?;
    let components = key.components().collect::<Vec<_>>();

    let header = db.get_grid_header(&hash, LANG).await?;

    let mut buttons = vec![];

    if components.len() > 1 {
        buttons.append(&mut navigation(
            components.last().unwrap().as_os_str().to_str().unwrap(),
        ));
    }

    let buttons = InlineKeyboardMarkup::new(buttons);

    Ok((header, buttons))
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
