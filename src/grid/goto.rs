use std::path::PathBuf;

use super::{callback, Navigation};
use crate::{templates, Db, Lang};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn goto(
    key: PathBuf,
    // FIXME: this hack is supposed to prevent vies increment on renders after like
    visit: bool,
    mut db: Db,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key_str = key.to_str().unwrap();

    let components_count = key.components().count();

    let header = db.get_grid_header(key_str, Lang::Ru.as_str()).await?;

    let mut text = header.clone();
    let mut buttons = vec![];

    if db.is_data_entry(key_str).await? {
        if visit {
            db.inc_views(key_str).await?;
        }

        let data_entry = db.get_data_entry(key_str, Lang::Ru.as_str()).await?;

        let likes = data_entry.likes;

        text = {
            use templates::data_entry::Context;

            let context = Context {
                header: header.clone(),
                data_entry,
            };
            templates::data_entry::render(context, Lang::Ru)?
        };

        let previous_key = PathBuf::from_iter(key.components().take(components_count - 1));

        let navigation = Navigation::builder()
            .cur(key_str)
            .prev(previous_key.to_str().unwrap())
            .likes(likes)
            .build();

        buttons.append(&mut navigation.render());
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

        if components_count > 1 {
            let previous_key = PathBuf::from_iter(key.components().take(components_count - 1));

            let navigation = Navigation::builder()
                .cur(key_str)
                .prev(previous_key.to_str().unwrap())
                .build();

            buttons.append(&mut navigation.render());
        }

        text = format!("*{}*", text);
    }

    let buttons = InlineKeyboardMarkup::new(buttons);

    Ok((text, buttons))
}
