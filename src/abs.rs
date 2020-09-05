use std::collections::HashMap;

/*
 * Absolute axes
 */
lazy_static! {
    pub static ref ABS: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("X", 0x00);
        m.insert("Y", 0x01);
        m.insert("Z", 0x02);
        m.insert("RX", 0x03);
        m.insert("RY", 0x04);
        m.insert("RZ", 0x05);
        m.insert("THROTTLE", 0x06);
        m.insert("RUDDER", 0x07);
        m.insert("WHEEL", 0x08);
        m.insert("GAS", 0x09);
        m.insert("BRAKE", 0x0a);
        m.insert("HAT0X", 0x10);
        m.insert("HAT0Y", 0x11);
        m.insert("HAT1X", 0x12);
        m.insert("HAT1Y", 0x13);
        m.insert("HAT2X", 0x14);
        m.insert("HAT2Y", 0x15);
        m.insert("HAT3X", 0x16);
        m.insert("HAT3Y", 0x17);
        m.insert("PRESSURE", 0x18);
        m.insert("DISTANCE", 0x19);
        m.insert("TILT_X", 0x1a);
        m.insert("TILT_Y", 0x1b);
        m.insert("TOOL_WIDTH", 0x1c);
        m.insert("VOLUME", 0x20);
        m.insert("MISC", 0x28);
        m.insert("RESERVED", 0x2e);
        m.insert("MT_SLOT", 0x2f);
        m.insert("MT_TOUCH_MAJOR", 0x30);
        m.insert("MT_TOUCH_MINOR", 0x31);
        m.insert("MT_WIDTH_MAJOR", 0x32);
        m.insert("MT_WIDTH_MINOR", 0x33);
        m.insert("MT_ORIENTATION", 0x34);
        m.insert("MT_POSITION_X", 0x35);
        m.insert("MT_POSITION_Y", 0x36);
        m.insert("MT_TOOL_TYPE", 0x37);
        m.insert("MT_BLOB_ID", 0x38);
        m.insert("MT_TRACKING_ID", 0x39);
        m.insert("MT_PRESSURE", 0x3a);
        m.insert("MT_DISTANCE", 0x3b);
        m.insert("MT_TOOL_X", 0x3c);
        m.insert("MT_TOOL_Y", 0x3d);
        m.insert("MAX", 0x3f);
        m.insert("CNT", 0x40);
        m.shrink_to_fit();
        m
    };
}
