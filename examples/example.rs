use input_event_codes_hashmap::KEY;

fn main() {
    let keycode_name = "T";
    if let Some(keycode_value) = KEY.get(keycode_name) {
        println!("The u32 value for {} is {}", keycode_name, keycode_value);
    } else {
        println!("The u32 value could not be found");
    }
}
