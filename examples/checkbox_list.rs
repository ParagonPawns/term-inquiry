fn main() {
    let result = CheckboxList::<&'static str>::new(String::from("Please select an option:"))
        .add_item("Option 1", "version 2.4")
        .add_item("Option 2", "version 2.5")
        .add_item("Option 3", "version 2.6")
        .render();

    match result {
        Ok(..) => {},
        Err(error) => {
            match error {
                InqueryMessage::CloseRequested => return,
                _ => panic!(
                    "There was an other error encoutered that shouldn't \
                    happen."
                )
            }
        }
    }
}

use term_menu::{ CheckboxList, InqueryMessage };
