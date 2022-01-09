use std::collections::VecDeque;

use super::callback;

use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardButtonKind};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, TypedBuilder)]
pub struct Navigation {
    #[builder(setter(into))]
    cur: String,
    #[builder(setter(into))]
    prev: String,
    #[builder(default, setter(strip_option, into))]
    likes: Option<u64>,
}

impl Navigation {
    pub fn render(&self) -> Vec<Vec<InlineKeyboardButton>> {
        let mut buttons = VecDeque::from([
            InlineKeyboardButton::new(
                "<<".to_string(),
                InlineKeyboardButtonKind::CallbackData(callback::data(
                    callback::Command::Goto,
                    "/",
                )),
            ),
            InlineKeyboardButton::new(
                "<".to_string(),
                InlineKeyboardButtonKind::CallbackData(callback::data(
                    callback::Command::Goto,
                    &self.prev,
                )),
            ),
        ]);

        if let Some(likes) = self.likes {
            buttons.push_front(InlineKeyboardButton::new(
                format!("👍 {}", likes),
                InlineKeyboardButtonKind::CallbackData(callback::data(
                    callback::Command::Like,
                    &self.cur,
                )),
            ));
        }

        vec![Vec::from(buttons)]
    }
}
