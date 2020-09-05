use std::collections::HashMap;

/*
 * Relative axes
 */
lazy_static! {
    pub static ref REL: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("X", 0x00);
        m.insert("Y", 0x01);
        m.insert("Z", 0x02);
        m.insert("RX", 0x03);
        m.insert("RY", 0x04);
        m.insert("RZ", 0x05);
        m.insert("HWHEEL", 0x06);
        m.insert("DIAL", 0x07);
        m.insert("WHEEL", 0x08);
        m.insert("MISC", 0x09);
        m.insert("RESERVED", 0x0a);
        m.insert("WHEEL_HI_RES", 0x0b);
        m.insert("HWHEEL_HI_RES", 0x0c);
        m.insert("MAX", 0x0f);
        m.insert("CNT", 0x10);
        m.shrink_to_fit();
        m
    };
}
