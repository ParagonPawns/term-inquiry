# Term Inquiry
Term Inquiry is a crate that allows creating various types of inquiries. This
projecte is still a work in progress.

## Supported Platforms
 * Linux
 * OSX
 * Windows (tested on Windows 10 Powershell and nushell)

## Inquiry Types
All inquiry types will have an example in the `examples` folder. So feel free
to try them out.

### List
Provides a list of single choise options with a given message.
```rust
use term_inquiry::List;

List::<&'static str>::new("Please select an option")
    .add_item("Option 1", "value1")
    .add_item("Option 2", "value2")
    .add_item("Option 3", "value3")
    .inquire();
```
output
```
[?] Please select an option:
  →  Option 1
    Option 2
    Option 3
```

### CheckboxList
Provides a list of check boxes (multiple choise) with a given message.
```rust
CheckboxList::<&'static str>::new("Please select an option:")
    .add_item("Option 1", "value1")
    .add_item("Option 2", "value2")
    .add_item("Option 3", "value3")
    .inquire();
```
output
```
[?] Please select an option: Press 'a' to accept selection
  →  [✘] Option 1
    [✘] Option 2
    [✘] Option 2
```
