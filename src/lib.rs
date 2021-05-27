mod checkbox_list;
mod list;
mod term_data;

pub use checkbox_list::CheckboxList as CheckboxList;
pub use list::List as List;

pub enum InqueryMessage {
    CloseRequested,
    FlushLockErr,
    TermDisableRawErr,
    TermEnableRawErr,
}

pub enum Keys {
    A,
    Up,
    Down,
    Left,
    Right,
    CtrlC,
    CtrlZ,
    Escape,
    Enter,
    Unhandled([u8;4]),
}

impl From<Stdin> for Keys {
    fn from(mut std_in: Stdin) -> Self {
        let mut data = [0, 0, 0, 0];
        let bytes_read = match std_in.read(&mut data) {
            Ok(bytes) => bytes,
            Err(error) =>
                panic!("There was an issue when reading bytes from std input \
                       stream. {}", error)
        };

        match data[0] {
            3 => Self::CtrlC,
            26 => Self::CtrlZ,
            10 => Self::Enter,
            27 => {
                if bytes_read == 0 {
                    return Self::Escape
                }

                match data[1] {
                    91 => match data[2] {
                        65 => Self::Up,
                        66 => Self::Down,
                        67 => Self::Right,
                        68 => Self::Left,
                        _ =>  Self::Unhandled(data)
                    },
                    _ => Self::Unhandled(data)
                }

            },
            97 => Self::A,
            _ => Self::Unhandled(data)
        }
    }
}

use std::io::{ Read, Stdin };
