pub struct List<T> {
    message: String,
    selection_list: Vec<(String, T)>,
    term_data: TermData,
}

impl<T> List<T> {
    pub fn new(message: String) -> Self {
        Self {
            message,
            selection_list: Vec::new(),
            term_data: TermData::new(),
        }
    }

    pub fn add_item(mut self, selection_name: &str, item: T) -> Self {
        self.selection_list
            .push((String::from(selection_name), item));
        self
    }

    pub fn inquire(mut self) -> Result<T, InquiryMessage> {
        if !self.term_data.enable_raw() {
            return Err(InquiryMessage::TermEnableRawErr);
        }

        AnsiBuilder::new()
            .text("[")
            .color()
            .fg()
            .bright_green()
            .text("?")
            .reset_attributes()
            .text("] ")
            .text(&self.message)
            .cursor()
            .save()
            .cursor()
            .hide()
            .println();
        let mut selected_index = 0;

        let list_len = self.selection_list.len();
        for _ in 0..list_len - 1 {
            println!();
        }

        loop {
            AnsiBuilder::new().cursor().up(list_len).print();

            for i in 0..list_len {
                AnsiBuilder::new()
                    .text("\n\r")
                    .erase_line(ClearMode::EntireLine)
                    .print();

                if i == selected_index {
                    AnsiBuilder::new()
                        .color()
                        .fg()
                        .bright_green()
                        .style()
                        .bold()
                        .text("  →  ")
                        .reset_attributes()
                        .text(&self.selection_list[i].0)
                        .print();
                    continue;
                }

                AnsiBuilder::new()
                    .text("    ")
                    .color()
                    .fg()
                    .gray()
                    .text(&self.selection_list[i].0)
                    .reset_attributes()
                    .print();
            }

            match stdout().lock().flush() {
                Ok(..) => {}
                Err(..) => return Err(InquiryMessage::FlushLockErr),
            };

            let key = Keys::from(stdin());

            match key {
                Keys::Up => {
                    if selected_index > 0 {
                        selected_index -= 1
                    }
                }
                Keys::Down => {
                    if selected_index < self.selection_list.len() - 1 {
                        selected_index += 1;
                    }
                }
                Keys::Enter => {
                    let (name, value) = self.selection_list.remove(selected_index);

                    AnsiBuilder::new()
                        .cursor()
                        .restore()
                        .color()
                        .fg()
                        .blue()
                        .text(&format!(" {}", name))
                        .reset_attributes()
                        .println()
                        .cursor()
                        .save()
                        .erase_in_display(EraseMode::CursorToEnd)
                        .cursor()
                        .restore()
                        .cursor()
                        .show()
                        .print();

                    if !self.term_data.disable_raw() {
                        return Err(InquiryMessage::TermDisableRawErr);
                    }

                    return Ok(value);
                }
                Keys::CtrlC | Keys::CtrlZ => {
                    AnsiBuilder::new().cursor().show().print();

                    if !self.term_data.disable_raw() {
                        return Err(InquiryMessage::TermDisableRawErr);
                    }

                    return Err(InquiryMessage::CloseRequested);
                }
                // Uncomment to view missing key data that is not handled.
                // Keys::Unhandled(data) => {
                //     panic!("{}-{}-{}-{}", data[0], data[1], data[2], data[3])
                // },
                _ => { /* we do nothing and proceed with loop */ }
            }
        }
    }
}

use std::io::{stdin, stdout, Write};

use ansi_builder::{AnsiBuilder, ClearMode, EraseMode};

use crate::{term_data::TermData, InquiryMessage, Keys};
