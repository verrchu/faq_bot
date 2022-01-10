use super::{callback, feedback::add_feedback_button};
use crate::{Db, Lang};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn init(mut db: Db) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let key = "/";

    let header = db.get_grid_header(key, Lang::Ru.as_str()).await?;

    let next_buttons = db
        .get_next_buttons(key, Lang::Ru.as_str())
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

    let mut next_buttons = InlineKeyboardMarkup::new(next_buttons);
    add_feedback_button(&mut next_buttons);

    Ok((format!("*{}*", header), next_buttons))
}
