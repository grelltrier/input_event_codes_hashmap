use std::collections::HashMap;

/*
 * Sounds
 */
lazy_static! {
    pub static ref SND: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("CLICK", 0x00);
        m.insert("BELL", 0x01);
        m.insert("TONE", 0x02);
        m.insert("MAX", 0x07);
        m.insert("CNT", 0x08);
        m.shrink_to_fit();
        m
    };
}
