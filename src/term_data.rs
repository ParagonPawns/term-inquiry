#[cfg(unix)]
pub struct TermData {
    fd: RawFd,
    old_term: Termios,
}

#[cfg(unix)]
impl TermData {
    pub fn new() -> Self {
        let fd = stdin().as_raw_fd();
        let old_term = match Termios::from_fd(fd) {
            Ok(term) => term,
            Err(error) => panic!("Failed to get fd from std in. {}", error)
        };

        Self {
            fd,
            old_term,
        }
    }

    pub fn enable_raw(&self) -> bool {
        let mut new_term = self.old_term.clone();

        new_term.c_lflag &= !(ECHO | ICANON | ISIG);

        if tcsetattr(self.fd, TCSAFLUSH, &new_term).is_err() {
            return false
        }

        true
    }

    pub fn disable_raw(&self) -> bool {
        match tcsetattr(self.fd, TCSAFLUSH, &self.old_term) {
            Ok(..) => true,
            Err(error) => {
                println!("{}", error);
                false
            }
        }
    }
}

#[cfg(unix)]
use std::os::unix::io::{ AsRawFd, RawFd };

use std::io::stdin;

#[cfg(unix)]
use termios::*;
