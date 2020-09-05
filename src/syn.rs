use std::collections::HashMap;

/*
 * Synchronization events.
 */
lazy_static! {
    pub static ref SYN: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("REPORT", 0);
        m.insert("CONFIG", 1);
        m.insert("MT_REPORT", 2);
        m.insert("DROPPED", 3);
        m.insert("MAX", 0xf);
        m.insert("CNT", 0xf + 1);
        m.shrink_to_fit();
        m
    };
}
