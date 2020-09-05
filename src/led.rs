use std::collections::HashMap;

/*
 * LEDs
 */
lazy_static! {
    pub static ref LED: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("NUML", 0x00);
        m.insert("CAPSL", 0x01);
        m.insert("SCROLLL", 0x02);
        m.insert("COMPOSE", 0x03);
        m.insert("KANA", 0x04);
        m.insert("SLEEP", 0x05);
        m.insert("SUSPEND", 0x06);
        m.insert("MUTE", 0x07);
        m.insert("MISC", 0x08);
        m.insert("MAIL", 0x09);
        m.insert("CHARGING", 0x0a);
        m.insert("MAX", 0x0f);
        m.insert("CNT", 0x10);
        m.shrink_to_fit();
        m
    };
}
