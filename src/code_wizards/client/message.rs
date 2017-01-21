use code_wizards::model::{Game, PlayerContext, Move};

#[derive(Debug, PartialEq)]
pub enum Message {
    UnknownMessage,
    GameOver,
    AuthenticationToken(String),
    TeamSize(i32),
    ProtocolVersion(i32),
    GameContext(Game),
    PlayerContext(Option<PlayerContext>),
    PlayerContextWithoutTrees(Option<PlayerContext>),
    MovesMessage(Vec<Move>),
}

impl Message {
    pub fn get_id(&self) -> i8 {
        match *self {
            Message::UnknownMessage => 0,
            Message::GameOver => 1,
            Message::AuthenticationToken(_) => 2,
            Message::TeamSize(_) => 3,
            Message::ProtocolVersion(_) => 4,
            Message::GameContext(_) => 5,
            Message::PlayerContext(_) => 6,
            Message::PlayerContextWithoutTrees(_) => 6,
            Message::MovesMessage(_) => 7,
        }
    }
}
