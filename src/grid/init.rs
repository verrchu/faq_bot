use crate::{utils, Db, Lang};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn init(mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key = "/";
    let hash = utils::hash(key);

    let header = db.get_grid_header(&hash, Lang::Ru).await?;

    let next_keys = db.get_next_keys(key).await?;
    let next_segments = next_keys
        .iter()
        .map(|next_key| {
            next_key
                .strip_prefix(&key)
                .map_err(anyhow::Error::from)
                .map(|next_key| next_key.to_str().unwrap())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let buttons = db
        .get_segment_names(&next_segments, Lang::Ru)
        .await?
        .into_iter()
        .zip(next_keys.into_iter())
        .map(|(name, key)| {
            vec![InlineKeyboardButton::new(
                name,
                InlineKeyboardButtonKind::CallbackData(cb_data(key.to_str().unwrap())),
            )]
        })
        .collect::<Vec<Vec<InlineKeyboardButton>>>();

    let buttons = InlineKeyboardMarkup::new(buttons);

    Ok((header, buttons))
}

fn cb_data(goto: &str) -> String {
    format!("/goto#{}", utils::hash(goto))
}
