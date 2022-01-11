use super::{callback, feedback::add_feedback_button};
use crate::{db, Context};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn init(mut context: Context) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    tracing::debug!("grid::init");

    let lang = context.lang;

    let key = "/";

    let header = db::grid::get_grid_header(&mut context.db, key, lang.as_str()).await?;

    let next_buttons = db::grid::get_next_buttons(&mut context.db, key, lang.as_str())
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
    add_feedback_button(&mut next_buttons, lang);

    Ok((format!("*{}*", header), next_buttons))
}
