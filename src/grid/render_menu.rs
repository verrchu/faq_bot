use std::path::PathBuf;

use super::{callback, feedback::add_feedback_button, Navigation};
use crate::{db, Context};

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind, InlineKeyboardMarkup};

pub async fn render_menu(
    key: PathBuf,
    context: Context,
) -> anyhow::Result<(String, InlineKeyboardMarkup)> {
    let lang = context.lang;

    let key_str = key.to_str().unwrap();

    tracing::info!(key = key_str, "grid::render_menu");

    let components_count = key.components().count();

    let header =
        db::grid::get_grid_header(context.db.clone(), key_str.to_string(), lang.to_string())
            .await?;

    let text = header.clone();
    let mut buttons =
        db::grid::get_next_buttons(context.db.clone(), key_str.to_string(), lang.to_string())
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

    if components_count > 1 {
        let previous_key = PathBuf::from_iter(key.components().take(components_count - 1));

        let navigation = Navigation::builder()
            .cur(key_str)
            .prev(previous_key.to_str().unwrap())
            .build();

        buttons.append(&mut navigation.render());
    }

    let mut buttons = InlineKeyboardMarkup::new(buttons);
    add_feedback_button(&mut buttons, lang);

    Ok((format!("*{text}*"), buttons))
}
