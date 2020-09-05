use std::collections::HashMap;

/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12
 * (see http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */
lazy_static! {
    pub static ref BTN: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("MISC", 0x100);
        m.insert("0", 0x100);
        m.insert("1", 0x101);
        m.insert("2", 0x102);
        m.insert("3", 0x103);
        m.insert("4", 0x104);
        m.insert("5", 0x105);
        m.insert("6", 0x106);
        m.insert("7", 0x107);
        m.insert("8", 0x108);
        m.insert("9", 0x109);
        m.insert("MOUSE", 0x110);
        m.insert("LEFT", 0x110);
        m.insert("RIGHT", 0x111);
        m.insert("MIDDLE", 0x112);
        m.insert("SIDE", 0x113);
        m.insert("EXTRA", 0x114);
        m.insert("FORWARD", 0x115);
        m.insert("BACK", 0x116);
        m.insert("TASK", 0x117);
        m.insert("JOYSTICK", 0x120);
        m.insert("TRIGGER", 0x120);
        m.insert("THUMB", 0x121);
        m.insert("THUMB2", 0x122);
        m.insert("TOP", 0x123);
        m.insert("TOP2", 0x124);
        m.insert("PINKIE", 0x125);
        m.insert("BASE", 0x126);
        m.insert("BASE2", 0x127);
        m.insert("BASE3", 0x128);
        m.insert("BASE4", 0x129);
        m.insert("BASE5", 0x12a);
        m.insert("BASE6", 0x12b);
        m.insert("DEAD", 0x12f);
        m.insert("GAMEPAD", 0x130);
        m.insert("SOUTH", 0x130);
        m.insert("A", 0x130);
        m.insert("EAST", 0x131);
        m.insert("B", 0x131);
        m.insert("C", 0x132);
        m.insert("NORTH", 0x133);
        m.insert("X", 0x133);
        m.insert("WEST", 0x134);
        m.insert("Y", 0x134);
        m.insert("Z", 0x135);
        m.insert("TL", 0x136);
        m.insert("TR", 0x137);
        m.insert("TL2", 0x138);
        m.insert("TR2", 0x139);
        m.insert("SELECT", 0x13a);
        m.insert("START", 0x13b);
        m.insert("MODE", 0x13c);
        m.insert("THUMBL", 0x13d);
        m.insert("THUMBR", 0x13e);
        m.insert("DIGI", 0x140);
        m.insert("TOOL_PEN", 0x140);
        m.insert("TOOL_RUBBER", 0x141);
        m.insert("TOOL_BRUSH", 0x142);
        m.insert("TOOL_PENCIL", 0x143);
        m.insert("TOOL_AIRBRUSH", 0x144);
        m.insert("TOOL_FINGER", 0x145);
        m.insert("TOOL_MOUSE", 0x146);
        m.insert("TOOL_LENS", 0x147);
        m.insert("TOOL_QUINTTAP", 0x148);
        m.insert("STYLUS3", 0x149);
        m.insert("TOUCH", 0x14a);
        m.insert("STYLUS", 0x14b);
        m.insert("STYLUS2", 0x14c);
        m.insert("TOOL_DOUBLETAP", 0x14d);
        m.insert("TOOL_TRIPLETAP", 0x14e);
        m.insert("TOOL_QUADTAP", 0x14f);
        m.insert("WHEEL", 0x150);
        m.insert("GEAR_DOWN", 0x150);
        m.insert("GEAR_UP", 0x151);
        m.insert("DPAD_UP", 0x220);
        m.insert("DPAD_DOWN", 0x221);
        m.insert("DPAD_LEFT", 0x222);
        m.insert("DPAD_RIGHT", 0x223);
        m.insert("TRIGGER_HAPPY", 0x2c0);
        m.insert("TRIGGER_HAPPY1", 0x2c0);
        m.insert("TRIGGER_HAPPY2", 0x2c1);
        m.insert("TRIGGER_HAPPY3", 0x2c2);
        m.insert("TRIGGER_HAPPY4", 0x2c3);
        m.insert("TRIGGER_HAPPY5", 0x2c4);
        m.insert("TRIGGER_HAPPY6", 0x2c5);
        m.insert("TRIGGER_HAPPY7", 0x2c6);
        m.insert("TRIGGER_HAPPY8", 0x2c7);
        m.insert("TRIGGER_HAPPY9", 0x2c8);
        m.insert("TRIGGER_HAPPY10", 0x2c9);
        m.insert("TRIGGER_HAPPY11", 0x2ca);
        m.insert("TRIGGER_HAPPY12", 0x2cb);
        m.insert("TRIGGER_HAPPY13", 0x2cc);
        m.insert("TRIGGER_HAPPY14", 0x2cd);
        m.insert("TRIGGER_HAPPY15", 0x2ce);
        m.insert("TRIGGER_HAPPY16", 0x2cf);
        m.insert("TRIGGER_HAPPY17", 0x2d0);
        m.insert("TRIGGER_HAPPY18", 0x2d1);
        m.insert("TRIGGER_HAPPY19", 0x2d2);
        m.insert("TRIGGER_HAPPY20", 0x2d3);
        m.insert("TRIGGER_HAPPY21", 0x2d4);
        m.insert("TRIGGER_HAPPY22", 0x2d5);
        m.insert("TRIGGER_HAPPY23", 0x2d6);
        m.insert("TRIGGER_HAPPY24", 0x2d7);
        m.insert("TRIGGER_HAPPY25", 0x2d8);
        m.insert("TRIGGER_HAPPY26", 0x2d9);
        m.insert("TRIGGER_HAPPY27", 0x2da);
        m.insert("TRIGGER_HAPPY28", 0x2db);
        m.insert("TRIGGER_HAPPY29", 0x2dc);
        m.insert("TRIGGER_HAPPY30", 0x2dd);
        m.insert("TRIGGER_HAPPY31", 0x2de);
        m.insert("TRIGGER_HAPPY32", 0x2df);
        m.insert("TRIGGER_HAPPY33", 0x2e0);
        m.insert("TRIGGER_HAPPY34", 0x2e1);
        m.insert("TRIGGER_HAPPY35", 0x2e2);
        m.insert("TRIGGER_HAPPY36", 0x2e3);
        m.insert("TRIGGER_HAPPY37", 0x2e4);
        m.insert("TRIGGER_HAPPY38", 0x2e5);
        m.insert("TRIGGER_HAPPY39", 0x2e6);
        m.insert("TRIGGER_HAPPY40", 0x2e7);
        m.shrink_to_fit();
        m
    };
}
