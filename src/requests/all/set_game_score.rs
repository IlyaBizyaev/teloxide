use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatOrInlineMessage, Message},
    Bot,
};

/// Use this method to set the score of the specified user in a game. On
/// success, if the message was sent by the bot, returns the edited Message,
/// otherwise returns True. Returns an error, if the new score is not greater
/// than the user's current score in the chat and force is False.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetGameScore<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    #[serde(flatten)]
    chat_or_inline_message: ChatOrInlineMessage,

    /// User identifier
    user_id: i32,
    /// New score, must be non-negative
    score: i32,
    /// Pass True, if the high score is allowed to decrease. This can be useful
    /// when fixing mistakes or banning cheaters
    force: Option<bool>,
    /// Pass True, if the game message should not be automatically edited to
    /// include the current scoreboard
    disable_edit_message: Option<bool>,
}

#[async_trait::async_trait]
impl Request for SetGameScore<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setGameScore",
            &self,
        )
        .await
    }
}

impl<'a> SetGameScore<'a> {
    pub(crate) fn new(
        bot: &'a Bot,
        chat_or_inline_message: ChatOrInlineMessage,
        user_id: i32,
        score: i32,
    ) -> Self {
        Self {
            bot,
            chat_or_inline_message,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
        }
    }

    pub fn chat_or_inline_message(mut self, val: ChatOrInlineMessage) -> Self {
        self.chat_or_inline_message = val;
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    pub fn score(mut self, val: i32) -> Self {
        self.score = val;
        self
    }

    pub fn force(mut self, val: bool) -> Self {
        self.force = Some(val);
        self
    }

    pub fn disable_edit_message(mut self, val: bool) -> Self {
        self.disable_edit_message = Some(val);
        self
    }
}
