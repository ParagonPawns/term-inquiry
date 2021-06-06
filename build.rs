fn main() {
    #[cfg(windows)]
    windows::build! {
        Windows::Win32::System::Diagnostics::Debug::{
            GetLastError,
        },
        Windows::Win32::System::Console::{
            CONSOLE_MODE,
            ENABLE_ECHO_INPUT,
            ENABLE_LINE_INPUT,
            ENABLE_PROCESSED_INPUT,
            ENABLE_VIRTUAL_TERMINAL_INPUT,
            GetConsoleMode,
            SetConsoleMode
        },
        Windows::Win32::System::SystemServices::{
            HANDLE
        }
    };
}
