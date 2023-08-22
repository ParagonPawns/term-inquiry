fn main() {
    let result = List::<&'static str>::new(String::from("Please select an option:"))
        .add_item("Option 1", "version 2.4")
        .add_item("Option 2", "version 2.5")
        .add_item("Option 3", "version 2.6")
        .inquire();

    match result {
        Ok(data) => println!("Option was selected got data: {}.", data),
        Err(error) => match error {
            InquiryMessage::CloseRequested => return,
            _ => panic!(
                "There was an other error encoutered that shouldn't \
                    happen."
            ),
        },
    }
}

use term_inquiry::{InquiryMessage, List};
