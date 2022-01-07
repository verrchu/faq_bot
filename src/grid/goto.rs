use std::path::Path;

use crate::{utils, Db};

use redis::Commands;
use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

static LANG: &str = "ru";

pub async fn goto(hash: String, mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key = db.get_key(&hash).await?;

    let header = db.get_grid_header(&hash, LANG).await?;

    let mut buttons = vec![];

    let mut next_buttons = db
        .get_next_keys(&key.to_str().unwrap())
        .await?
        .iter()
        .map(|next_key| {
            let last_segment = next_key
                .strip_prefix(&key)
                .map_err(anyhow::Error::from)
                .map(|next_key| next_key.to_str().unwrap())?;

            Ok(vec![InlineKeyboardButton::new(
                last_segment.to_string(),
                InlineKeyboardButtonKind::CallbackData(cb_data(next_key.to_str().unwrap())),
            )])
        })
        .collect::<anyhow::Result<Vec<Vec<InlineKeyboardButton>>>>()?;

    buttons.append(&mut next_buttons);

    if key.as_path() != Path::new("/") {
        buttons.append(&mut navigation(key.to_str().unwrap()));
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
