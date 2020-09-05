use std::collections::HashMap;

/*
 * Device properties and quirks
 */
lazy_static! {
    pub static ref INPUT_PROP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("POINTER", 0x00);
        m.insert("DIRECT", 0x01);
        m.insert("BUTTONPAD", 0x02);
        m.insert("SEMI_MT", 0x03);
        m.insert("TOPBUTTONPAD", 0x04);
        m.insert("POINTING_STICK", 0x05);
        m.insert("ACCELEROMETER", 0x06);
        m.insert("MAX", 0x1f);
        m.insert("CNT", 0x20);
        m.shrink_to_fit();
        m
    };
}
