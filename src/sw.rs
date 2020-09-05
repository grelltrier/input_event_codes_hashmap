use std::collections::HashMap;

/*
 * Switch events
 */
lazy_static! {
	pub static ref SW: HashMap<&'static str, u32> = {
		let mut m = HashMap::new();
		m.insert("LID", 0x00);
		m.insert("TABLET_MODE", 0x01);
		m.insert("HEADPHONE_INSERT", 0x02);
		m.insert("RFKILL_ALL", 0x03);
		m.insert("RADIO", 0x03);
		m.insert("MICROPHONE_INSERT", 0x04);
		m.insert("DOCK", 0x05);
		m.insert("LINEOUT_INSERT", 0x06);
		m.insert("JACK_PHYSICAL_INSERT", 0x07);
		m.insert("VIDEOOUT_INSERT", 0x08);
		m.insert("CAMERA_LENS_COVER", 0x09);
		m.insert("KEYPAD_SLIDE", 0x0a);
		m.insert("FRONT_PROXIMITY", 0x0b);
		m.insert("ROTATE_LOCK", 0x0c);
		m.insert("LINEIN_INSERT", 0x0d);
		m.insert("MUTE_DEVICE", 0x0e);
		m.insert("PEN_INSERTED", 0x0f);
		m.insert("MACHINE_COVER", 0x10);
		m.insert("MAX", 0x10);
		m.insert("CNT", 0x11);
		m.shrink_to_fit();
		m
	};
}
