use std::io;
use byteorder::{ByteOrder, WriteBytesExt};
use code_wizards::model::{ActionType, Game, LaneType, Message as ModelMessage, Move, PlayerContext,
                          SkillType};
use super::message::Message;

pub trait WriteMessage: WriteBytesExt {
    fn write_message<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        try!(self.write_message_id(value.get_id()));
        self.write_message_content::<B>(value)
    }

    fn write_message_id(&mut self, value: i8) -> io::Result<()> {
        self.write_i8(value)
    }

    fn write_message_content<B: ByteOrder>(&mut self, value: &Message) -> io::Result<()> {
        match value {
            &Message::UnknownMessage => self.write_message_content_unknown_message(),
            &Message::GameOver => self.write_message_content_game_over(),
            &Message::AuthenticationToken(ref v) => {
                self.write_message_content_authentication_token::<B>(v)
            }
            &Message::TeamSize(v) => self.write_message_content_team_size::<B>(v),
            &Message::ProtocolVersion(v) => self.write_message_content_protocol_version::<B>(v),
            &Message::GameContext(ref v) => self.write_message_content_game_context(v),
            &Message::PlayerContext(ref v) => self.write_message_content_player_context(v),
            &Message::PlayerContextWithoutTrees(ref v) => {
                self.write_message_content_player_context_without_trees(v)
            }
            &Message::MovesMessage(ref v) => self.write_message_content_moves_message::<B>(v),
        }
    }

    fn write_message_content_unknown_message(&mut self) -> io::Result<()> {
        unimplemented!()
    }

    fn write_message_content_game_over(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_message_content_authentication_token<B: ByteOrder>(&mut self, value: &String) -> io::Result<()> {
        try!(self.write_i32::<B>(value.len() as i32));

        for b in value.bytes() {
            try!(self.write_u8(b));
        }

        Ok(())
    }

    fn write_message_content_team_size<B: ByteOrder>(&mut self, value: i32) -> io::Result<()> {
        self.write_i32::<B>(value)
    }

    fn write_message_content_protocol_version<B: ByteOrder>(&mut self, value: i32) -> io::Result<()> {
        self.write_i32::<B>(value)
    }

    fn write_message_content_game_context(&mut self, _value: &Game) -> io::Result<()> {
        unimplemented!()
    }

    fn write_message_content_player_context(&mut self, _value: &Option<PlayerContext>) -> io::Result<()> {
        unimplemented!()
    }

    fn write_message_content_player_context_without_trees(&mut self, _value: &Option<PlayerContext>) -> io::Result<()> {
        unimplemented!()
    }

    fn write_message_content_moves_message<B: ByteOrder>(&mut self, value: &Vec<Move>) -> io::Result<()> {
        self.write_vec_move::<B>(value)
    }

    fn write_vec_move<B: ByteOrder>(&mut self, value: &Vec<Move>) -> io::Result<()> {
        self.write_vec::<B, _, _>(value, |s, v| s.write_move::<B>(v))
    }

    fn write_vec_model_message<B: ByteOrder>(&mut self, value: &Vec<ModelMessage>) -> io::Result<()> {
        self.write_vec::<B, _, _>(value, |s, v| s.write_model_message::<B>(v))
    }

    fn write_vec_i8<B: ByteOrder>(&mut self, value: &Vec<i8>) -> io::Result<()> {
        self.write_vec::<B, _, _>(value, |s, v| s.write_i8(*v))
    }

    fn write_vec<B: ByteOrder, T, F>(&mut self, values: &Vec<T>, write: F) -> io::Result<()>
            where F: Fn(&mut Self, &T) -> io::Result<()> {
        try!(self.write_i32::<B>(values.len() as i32));

        for v in values.iter() {
            try!(write(self, v));
        }

        Ok(())
    }

    fn write_move<B: ByteOrder>(&mut self, value: &Move) -> io::Result<()> {
        try!(self.write_bool(true));
        try!(self.write_f64::<B>(value.speed()));
        try!(self.write_f64::<B>(value.strafe_speed()));
        try!(self.write_f64::<B>(value.turn()));
        try!(self.write_action_type(value.action()));
        try!(self.write_f64::<B>(value.cast_angle()));
        try!(self.write_f64::<B>(value.min_cast_distance()));
        try!(self.write_f64::<B>(value.max_cast_distance()));
        try!(self.write_i64::<B>(value.status_target_id()));
        try!(self.write_skill_type(value.skill_to_learn()));
        try!(self.write_vec_model_message::<B>(&value.messages()));
        Ok(())
    }

    fn write_action_type(&mut self, value: ActionType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_skill_type(&mut self, value: SkillType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_lane_type(&mut self, value: LaneType) -> io::Result<()> {
        self.write_i8(value as i8)
    }

    fn write_model_message<B: ByteOrder>(&mut self, value: &ModelMessage) -> io::Result<()> {
        try!(self.write_bool(true));
        try!(self.write_lane_type(value.lane));
        try!(self.write_skill_type(value.skill_to_learn));
        try!(self.write_vec_i8::<B>(&value.raw_message));
        Ok(())
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(if value { 1 } else { 0 })
    }
}

impl<W: WriteBytesExt> WriteMessage for W {}

#[test]
fn test_write_bool_false() {
    use std::io::Cursor;
    {
        let mut buffer = vec![];
        buffer.write_bool(false).unwrap();
        assert_eq!(buffer, vec![0u8]);
    }
}

#[test]
fn test_write_bool_true() {
    use std::io::Cursor;
    {
        let mut buffer = vec![];
        buffer.write_bool(true).unwrap();
        assert_eq!(buffer, vec![1u8]);
    }
}
