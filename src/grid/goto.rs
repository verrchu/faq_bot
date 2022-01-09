use std::path::PathBuf;

use super::callback;
use crate::{templates, Db, Lang};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn goto(key: PathBuf, mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key_str = key.to_str().unwrap();

    let components_count = key.components().count();

    let header = db.get_grid_header(key_str, Lang::Ru.as_str()).await?;

    let mut text = header.clone();
    let mut buttons = vec![];

    if db.is_data_entry(key_str).await? {
        db.inc_views(key_str).await?;

        let data_entry = db.get_data_entry(key_str, Lang::Ru.as_str()).await?;

        text = {
            use templates::data_entry::Context;

            let context = Context {
                header: header.clone(),
                data_entry,
            };
            templates::data_entry::render(context, Lang::Ru)?
        };
    } else {
        let next_buttons = db
            .get_next_buttons(key_str, Lang::Ru.as_str())
            .await?
            .into_iter()
            .map(|(key, name)| {
                vec![InlineKeyboardButton::new(
                    &name,
                    InlineKeyboardButtonKind::CallbackData(callback::data(
                        callback::Command::Goto,
                        &key,
                    )),
                )]
            })
            .collect::<Vec<Vec<InlineKeyboardButton>>>();

        buttons = next_buttons;
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
