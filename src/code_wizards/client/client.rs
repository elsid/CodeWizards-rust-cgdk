use std::io;
use byteorder::ByteOrder;
use code_wizards::model::Tree;

const PROTOCOL_VERSION: i32 = 1;

pub fn run<'r, B: ByteOrder>(host: &'r str, port: u16, token: String) -> io::Result<()> {
    use std::io::{Error, ErrorKind, Write};
    use std::net::TcpStream;
    use code_wizards::model::Move;
    use code_wizards::MyStrategy;
    use super::message::Message;
    use super::read_message::ReadMessage;
    use super::write_message::WriteMessage;

    let mut stream = try!(TcpStream::connect((host, port)));

    try!(stream.set_nodelay(true));

    try!(stream.write_message::<B>(&Message::AuthenticationToken(token.clone())));
    try!(stream.write_message::<B>(&Message::ProtocolVersion(PROTOCOL_VERSION)));

    let team_size = match try!(stream.read_message::<B>()) {
        Message::TeamSize(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::TeamSize, but received: {:?}", v))),
    };

    if team_size < 0 {
        return Err(Error::new(ErrorKind::Other, format!("Team size < 0: {}", team_size)));
    }

    let game = match try!(stream.read_message::<B>()) {
        Message::GameContext(v) => v,
        v => return Err(Error::new(ErrorKind::Other, format!("Expected Message::GameContext, but received: {:?}", v))),
    };

    let mut strategies = vec![MyStrategy::new(); team_size as usize];
    let mut trees = vec![];

    loop {
        let player_context = match try!(stream.read_message::<B>()) {
            Message::GameOver => break,
            Message::PlayerContext(v) => {
                if v.is_some() {
                    trees = v.as_ref().unwrap().world.trees.clone();
                }
                v
            }
            Message::PlayerContextWithoutTrees(mut v) => {
                if v.is_some() {
                    v.as_mut().unwrap().world.trees = trees.clone();
                }
                v
            }
            v => return Err(Error::new(ErrorKind::Other,
                                       format!("Expected Message::GameOver, \
                                                Message::PlayerContext or \
                                                Message::PlayerContextWithoutTrees, but \
                                                received: {:?}", v)))
        };

        if let Some(v) = player_context {
            let mut moves = vec![Move::new(); team_size as usize];

            for i in 0..team_size as usize {
                strategies[i].move_(&v.wizards[i], &v.world, &game, &mut moves[i]);
            }

            try!(stream.write_message::<B>(&Message::MovesMessage(moves)));
        } else {
            break;
        }
    }

    Ok(())
}
