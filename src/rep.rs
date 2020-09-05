use std::collections::HashMap;

/*
 * Autorepeat values
 */
lazy_static! {
    pub static ref REP: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("DELAY", 0x00);
        m.insert("PERIOD", 0x01);
        m.insert("MAX", 0x01);
        m.insert("CNT", 0x02);
        m.shrink_to_fit();
        m
    };
}
