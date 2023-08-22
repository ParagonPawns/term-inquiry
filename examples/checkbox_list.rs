fn main() {
    let result = CheckboxList::<&'static str>::new("Please select an option:")
        .add_item("Option 1", "version 2.4")
        .add_item("Option 2", "version 2.5")
        .add_item("Option 3", "version 2.6")
        .inquire();

    match result {
        Ok(..) => {}
        Err(error) => match error {
            InquiryMessage::CloseRequested => return,
            _ => panic!(
                "There was an other error encoutered that shouldn't \
                    happen."
            ),
        },
    }
}

use term_inquiry::{CheckboxList, InquiryMessage};
