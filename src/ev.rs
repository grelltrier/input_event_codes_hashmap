use std::collections::HashMap;

/*
 * Event types
 */
lazy_static! {
    pub static ref EV: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("SYN", 0x00);
        m.insert("KEY", 0x01);
        m.insert("REL", 0x02);
        m.insert("ABS", 0x03);
        m.insert("MSC", 0x04);
        m.insert("SW", 0x05);
        m.insert("LED", 0x11);
        m.insert("SND", 0x12);
        m.insert("REP", 0x14);
        m.insert("FF", 0x15);
        m.insert("PWR", 0x16);
        m.insert("FF_STATUS", 0x17);
        m.insert("MAX", 0x1f);
        m.insert("CNT", 0x20);
        m.shrink_to_fit();
        m
    };
}
