use std::collections::HashMap;

/*
 * Misc events
 */
lazy_static! {
    pub static ref MSC: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("SERIAL", 0x00);
        m.insert("PULSELED", 0x01);
        m.insert("GESTURE", 0x02);
        m.insert("RAW", 0x03);
        m.insert("SCAN", 0x04);
        m.insert("TIMESTAMP", 0x05);
        m.insert("MAX", 0x07);
        m.insert("CNT", 0x08);
        m.shrink_to_fit();
        m
    };
}
