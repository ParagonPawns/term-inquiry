struct Item<T> {
    message: String,
    value: T,
    selected: bool,
}

pub struct CheckboxList<T> {
    message: String,
    selection_list: Vec<Item<T>>,
    term_data: TermData,
}

impl<T> CheckboxList<T> {
    pub fn new(message: String) -> Self {
        Self {
            message,
            selection_list: Vec::new(),
            term_data: TermData::new()
        }
    }

    pub fn add_item(mut self, selection_name: &str, item: T) -> Self {
        let item = Item::<T> {
            message: String::from(selection_name),
            value: item,
            selected: false,
        };

        self.selection_list.push(item);
        self
    }

    pub fn inquire(mut self) -> Result<Vec<T>, InquiryMessage> {
        if !self.term_data.enable_raw() {
            return Err(InquiryMessage::TermEnableRawErr);
        }

        AnsiBuilder::new()
            .text("[")
            .color().fg().bright_green()
            .text("?")
            .reset_attributes()
            .text("] ")
            .text(&self.message)
            .cursor().save()
            .cursor().hide()
            .color().fg().gray()
            .text(" Press 'a' to accept selection")
            .println();
        let mut selected_index = 0;

        let list_len = self.selection_list.len();
        for _ in 0..list_len - 1 {
            println!();
        }

        loop {
            AnsiBuilder::new()
                .cursor().up(list_len)
                .print();

            for i in 0..list_len {
                AnsiBuilder::new()
                    .text("\n\r")
                    .erase_line(ClearMode::EntireLine)
                    .print();

                if i == selected_index {
                    AnsiBuilder::new()
                        .color().fg().bright_green()
                        .style().bold()
                        .text("  →  ")
                        .reset_attributes()
                        .print();

                    CheckboxList::render_item(&self.selection_list[i]);
                    continue
                }

                AnsiBuilder::new().text("    ")
                    .print();

                CheckboxList::render_item(&self.selection_list[i]);
            }

            match stdout().lock().flush() {
                Ok(..) => {},
                Err(..) => return Err(InquiryMessage::FlushLockErr)
            };

            let key = Keys::from(stdin());

            match key {
                Keys::Up => if selected_index > 0 { selected_index -= 1 },
                Keys::Down => if selected_index < self.selection_list.len() - 1 {
                    selected_index += 1;
                },
                Keys::Enter => {
                    self.selection_list[selected_index].selected =
                        !self.selection_list[selected_index].selected;
                }
                Keys::A => {
                    let mut selected_items = Vec::new();
                    let mut selected_names = String::new();

                    let mut i = 0;
                    while i < self.selection_list.len() {
                        if !self.selection_list[i].selected {
                            i += 1;
                            continue
                        }

                        selected_names.push_str(&(self.selection_list[i].message.clone() + ", "));
                        selected_items.push(self.selection_list.remove(i).value);
                    }

                    selected_names.pop();
                    selected_names.pop();

                    AnsiBuilder::new()
                        .cursor().restore()
                        .color().fg().blue()
                        .text(&format!(" {}", selected_names))
                        .reset_attributes()
                        .cursor().save()
                        .erase_in_display(EraseMode::CursorToEnd)
                        .cursor().restore()
                        .cursor().show()
                        .println();

                    return Ok(selected_items)
                },
                Keys::CtrlC | Keys::CtrlZ => {
                    AnsiBuilder::new()
                        .cursor().show()
                        .print();

                    if !self.term_data.disable_raw() {
                        return Err(InquiryMessage::TermDisableRawErr)
                    }

                    return Err(InquiryMessage::CloseRequested)
                },
                 // Uncomment to view missing key data that is not handled.
                 // Keys::Unhandled(data) => {
                 //     panic!("{}-{}-{}-{}", data[0], data[1], data[2], data[3])
                 // },
                _ => {/* we do nothing and proceed with loop */}
            }
        }
    }

    fn render_item(item: &Item<T>) {
        if item.selected {
            AnsiBuilder::new()
                .text("[")
                .color().fg().bright_green()
                .text("✓")
                .reset_attributes()
                .text("] ")
                .text(&item.message)
                .print();
            return
        }

        AnsiBuilder::new()
            .text("[")
            .color().fg().bright_red()
            .text("✘")
            .reset_attributes()
            .text("] ")
            .text(&item.message)
            .print();
    }
}

use std::io::{ Write, stdin, stdout };

use ansi_builder::{ AnsiBuilder, ClearMode, EraseMode };

use crate::{ InquiryMessage, Keys, term_data::TermData };
