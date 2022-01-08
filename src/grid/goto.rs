use std::path::PathBuf;

use super::{callback, utils::format_segment};
use crate::{templates, utils, Db, Lang};

use futures::try_join;
use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn goto(key: PathBuf, mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key_str = key.to_str().unwrap();

    let components_count = key.components().count();

    let header = db.get_grid_header(key_str, &Lang::Ru).await?;

    let mut text = header.clone();
    let mut buttons = vec![];

    if db.is_data_entry(key_str).await? {
        let data = db
            .get_key_data(key_str, &Lang::Ru)
            .await?
            .trim()
            .to_string();
        let created = db
            .get_key_created(key_str)
            .await
            .map(utils::unixtime_to_datetime)?;
        let views = db.inc_views(key_str).await?;

        text = {
            use templates::data_entry::Context;

            let context = Context {
                header: header.clone(),
                data,
                created,
                views,
            };
            templates::data_entry::render(context, Lang::Ru)?
        };
    } else {
        let next_keys = db.get_next_keys(key_str).await?;
        let next_segments = next_keys
            .iter()
            .map(|next_key| {
                next_key
                    .strip_prefix(&key)
                    .map_err(anyhow::Error::from)
                    .map(|next_segment| next_segment.to_str().unwrap().to_string())
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let next_keys = next_keys
            .into_iter()
            .map(|next_key| next_key.to_str().unwrap().to_string())
            .collect::<Vec<_>>();

        let (segment_names, key_icons) = {
            let mut db1 = db.clone();
            let mut db2 = db.clone();

            try_join!(
                db1.get_segment_names(&next_segments, &Lang::Ru),
                db2.get_key_icons(next_keys.clone()),
            )
        }?;

        let mut next_buttons = segment_names
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

        buttons.append(&mut next_buttons);
    }

    if components_count > 1 {
        let previous_key = PathBuf::from_iter(key.components().take(components_count - 1));
        buttons.append(&mut navigation(previous_key.to_str().unwrap()));
    }

    let buttons = InlineKeyboardMarkup::new(buttons);

    Ok((text, buttons))
}

fn navigation(back: &str) -> Vec<Vec<InlineKeyboardButton>> {
    vec![vec![
        InlineKeyboardButton::new(
            "<<".to_string(),
            InlineKeyboardButtonKind::CallbackData(callback::data(callback::Command::Goto, "/")),
        ),
        InlineKeyboardButton::new(
            "<".to_string(),
            InlineKeyboardButtonKind::CallbackData(callback::data(callback::Command::Goto, back)),
        ),
    ]]
}
