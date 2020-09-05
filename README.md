# input_event_codes_hashmap
This crate provides HashMaps to look up the u32 value for a given input event code. The lazy-static crate is used so they are only created, if you used them.
The codes are taken from input-event-codes.h.

## Usage
There are the HashMaps ABS, BTN, EV, INPUT_PROP, KEY, LED, MSC, REL, REP, SND, SW and SYN. The prefix tells you which HashMap to use. 
If you are looking for KEY_T, you want to search in the KEY HashMap for "T". No need to repeat the prefix. There is an example on how to find the u32 value for KEY_T.
It's as easy as:
```Rust
use input_event_codes_hashmap::KEY;

fn main() {
    let keycode_name = "T";
    if let Some(keycode_value) = KEY.get(keycode_name) {
        println!("The u32 value for {} is {}", keycode_name, keycode_value);
    } else {
        println!("The u32 value could not be found");
    }
}
```
You can run the example by entering the following command:
```bash
cargo run --example example
```

## Contributing
PRs are always welcome! If I made a mistake please tell me as well :)
