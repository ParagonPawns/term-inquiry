#[cfg(windows)]
pub struct TermData {
    std_handle: HANDLE,
    original_mode: CONSOLE_MODE,
}

#[cfg(windows)]
impl TermData {
    pub fn new() -> Self {
        Self {
            std_handle: HANDLE(stdin().as_raw_handle() as isize),
            original_mode: CONSOLE_MODE::default(),
        }
    }

    pub fn enable_raw(&mut self) -> bool {
        unsafe {
            if !GetConsoleMode(self.std_handle, &mut self.original_mode).as_bool() {
                return false
            }

            let mut current_mode = self.original_mode | ENABLE_VIRTUAL_TERMINAL_INPUT;
            current_mode &= CONSOLE_MODE(!(ENABLE_PROCESSED_INPUT | ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT).0);
            if !SetConsoleMode(self.std_handle, current_mode).as_bool() {
                let error = GetLastError();
                println!("Error - {:?}", error);
                return false
            }
        }
        true
    }

    pub fn disable_raw(&self) -> bool {
        unsafe {
            if !SetConsoleMode(self.std_handle, self.original_mode).as_bool() {
                return false
            }
        }
        true
    }
}

#[cfg(windows)]
use crate::Windows::Win32::System::{
    Diagnostics::Debug::GetLastError,
    Console::{
        CONSOLE_MODE,
        ENABLE_ECHO_INPUT,
        ENABLE_LINE_INPUT,
        ENABLE_PROCESSED_INPUT,
        ENABLE_VIRTUAL_TERMINAL_INPUT,
        GetConsoleMode,
        SetConsoleMode
    },
    SystemServices::HANDLE,
};

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;

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

#[cfg(unix)]
use termios::*;

use std::io::stdin;
