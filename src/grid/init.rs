use super::{callback, utils::format_segment};
use crate::{Db, Lang};

use futures::try_join;
use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn init(mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key = "/";

    let header = db.get_grid_header(key, &Lang::Ru).await?;

    let next_keys = db.get_next_keys(key).await?;
    let next_segments = next_keys
        .iter()
        .map(|next_key| {
            next_key
                .strip_prefix(&key)
                .map_err(anyhow::Error::from)
                .map(|next_key| next_key.to_str().unwrap().to_string())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let next_keys = next_keys
        .into_iter()
        .map(|key| key.to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let (segment_names, key_icons) = {
        let mut db1 = db.clone();
        let mut db2 = db.clone();

        try_join!(
            db1.get_segment_names(&next_segments, &Lang::Ru),
            db2.get_key_icons(next_keys.clone()),
        )
    }?;

    let next_buttons = segment_names
        .iter()
        .zip(key_icons.iter())
        .zip(next_keys.iter())
        .map(|((name, icon), key)| {
            vec![InlineKeyboardButton::new(
                format_segment(name, icon.as_ref()),
                InlineKeyboardButtonKind::CallbackData(callback::data(
                    callback::Command::Goto,
                    key,
                )),
            )]
        })
        .collect::<Vec<Vec<InlineKeyboardButton>>>();

    let next_buttons = InlineKeyboardMarkup::new(next_buttons);

    Ok((header, next_buttons))
}
