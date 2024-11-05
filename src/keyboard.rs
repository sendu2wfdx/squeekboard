/*! State of the emulated keyboard and keys.
 * Regards the keyboard as if it was composed of switches. */

use crate::action::Action;
use crate::layout;
use crate::util;
use crate::keycodes::{*};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::mem;
use std::ptr;
use std::string::FromUtf8Error;

// Traits
use std::io::Write;
use std::iter::{ FromIterator, IntoIterator };

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PressType {
    Released = 0,
    Pressed = 1,
}

/// The extended, unambiguous layout-keycode
#[derive(Debug, Clone, PartialEq)]
pub struct KeyCode {
    pub code: u32,
    pub keymap_idx: usize,
}

bitflags!{
    /// Map to `virtual_keyboard.modifiers` modifiers values
    /// From https://www.x.org/releases/current/doc/kbproto/xkbproto.html#Keyboard_State
    pub struct Modifiers: u8 {
        const SHIFT = 0x1;
        const LOCK = 0x2;
        const CONTROL = 0x4;
        /// Alt
        const MOD1 = 0x8;
        const MOD2 = 0x10;
        const MOD3 = 0x20;
        /// Meta
        const MOD4 = 0x40;
        /// AltGr
        const MOD5 = 0x80;
    }
}

/// When the submitted actions of keys need to be tracked,
/// they need a stable, comparable ID.
/// With layout::ButtonPosition, the IDs are unique within layouts.
#[derive(Clone, PartialEq)]
pub struct KeyStateId(layout::ButtonPosition);

impl From<&layout::ButtonPosition> for KeyStateId {
    fn from(v: &layout::ButtonPosition) -> Self {
        Self(v.clone())
    }
}

#[derive(Clone)]
pub struct Key {
    /// A cache of raw keycodes derived from Action::Submit given a keymap
    pub keycodes: Vec<KeyCode>,
    /// Static description of what the key does when pressed or released
    pub action: Action,
}

#[derive(Debug, Clone)]
pub struct KeyState {
    pub pressed: PressType,
}

impl KeyState {
    #[must_use]
    pub fn into_released(self) -> KeyState {
        KeyState {
            pressed: PressType::Released,
            ..self
        }
    }

    #[must_use]
    pub fn into_pressed(self) -> KeyState {
        KeyState {
            pressed: PressType::Pressed,
            ..self
        }
    }
}

/// Sorts an iterator by converting it to a Vector and back
fn sorted<'a, I: Iterator<Item=String>>(
    iter: I
) -> impl Iterator<Item=String> {
    let mut v: Vec<String> = iter.collect();
    v.sort();
    v.into_iter()
}

/// Generates a mapping where each key gets a keycode, starting from ~~8~~
/// HACK: starting from 9, because 8 results in keycode 0,
/// which the compositor likes to discard
pub fn generate_keycodes<'a, C: IntoIterator<Item=String>>(
    key_names: C,
) -> HashMap<String, KeyCode> {
    // Some clients try to interpret keymaps as if they were input-sequences coming from evdev.
    // Workaround: Only use keycodes which directly produce characters.
    let allowed = [KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8,
                   KEY_9, KEY_0, KEY_MINUS, KEY_EQUAL, KEY_Q, KEY_W, KEY_E,
                   KEY_R, KEY_T, KEY_Y, KEY_U, KEY_I, KEY_O, KEY_P, KEY_LEFTBRACE,
                   KEY_RIGHTBRACE, KEY_A, KEY_S, KEY_D, KEY_F, KEY_G, KEY_H,
                   KEY_J, KEY_K, KEY_L, KEY_SEMICOLON, KEY_APOSTROPHE, KEY_GRAVE,
                   KEY_BACKSLASH, KEY_Z, KEY_X, KEY_C, KEY_V, KEY_B, KEY_N,
                   KEY_M, KEY_COMMA, KEY_DOT, KEY_SLASH];

    let keycode_offset = 8;

    let mut keycode_map = HashMap::from_iter(
        // Sort to remove a source of indeterminism in keycode assignment.
        sorted(key_names.into_iter())
            .zip(util::cycle_count((9..255).filter(|x| allowed.contains(&(x - keycode_offset)))))
            .map(|(name, (mut code, mut keymap_idx))| {
                // Some apps expect specific keycodes for certain characters/functions.
                // Reserve the first 2 keymaps (0 and 1) for sorting those manually,
                // and use keymap_idx 2 and higher for other keycodes,
                // to not assign identical keycodes twice.
                // TODO: Add the "Shift"-modifier for keycodes on keymap 1.
                keymap_idx = keymap_idx + 2;
                if name == "1"         { code = KEY_1           + keycode_offset; keymap_idx = 0 }
                if name == "2"         { code = KEY_2           + keycode_offset; keymap_idx = 0 }
                if name == "3"         { code = KEY_3           + keycode_offset; keymap_idx = 0 }
                if name == "4"         { code = KEY_4           + keycode_offset; keymap_idx = 0 }
                if name == "5"         { code = KEY_5           + keycode_offset; keymap_idx = 0 }
                if name == "6"         { code = KEY_6           + keycode_offset; keymap_idx = 0 }
                if name == "7"         { code = KEY_7           + keycode_offset; keymap_idx = 0 }
                if name == "8"         { code = KEY_8           + keycode_offset; keymap_idx = 0 }
                if name == "9"         { code = KEY_9           + keycode_offset; keymap_idx = 0 }
                if name == "0"         { code = KEY_0           + keycode_offset; keymap_idx = 0 }
                if name == "a"         { code = KEY_A           + keycode_offset; keymap_idx = 0 }
                if name == "b"         { code = KEY_B           + keycode_offset; keymap_idx = 0 }
                if name == "c"         { code = KEY_C           + keycode_offset; keymap_idx = 0 }
                if name == "d"         { code = KEY_D           + keycode_offset; keymap_idx = 0 }
                if name == "e"         { code = KEY_E           + keycode_offset; keymap_idx = 0 }
                if name == "f"         { code = KEY_F           + keycode_offset; keymap_idx = 0 }
                if name == "g"         { code = KEY_G           + keycode_offset; keymap_idx = 0 }
                if name == "h"         { code = KEY_H           + keycode_offset; keymap_idx = 0 }
                if name == "i"         { code = KEY_I           + keycode_offset; keymap_idx = 0 }
                if name == "j"         { code = KEY_J           + keycode_offset; keymap_idx = 0 }
                if name == "k"         { code = KEY_K           + keycode_offset; keymap_idx = 0 }
                if name == "l"         { code = KEY_L           + keycode_offset; keymap_idx = 0 }
                if name == "m"         { code = KEY_M           + keycode_offset; keymap_idx = 0 }
                if name == "n"         { code = KEY_N           + keycode_offset; keymap_idx = 0 }
                if name == "o"         { code = KEY_O           + keycode_offset; keymap_idx = 0 }
                if name == "p"         { code = KEY_P           + keycode_offset; keymap_idx = 0 }
                if name == "q"         { code = KEY_Q           + keycode_offset; keymap_idx = 0 }
                if name == "r"         { code = KEY_R           + keycode_offset; keymap_idx = 0 }
                if name == "s"         { code = KEY_S           + keycode_offset; keymap_idx = 0 }
                if name == "t"         { code = KEY_T           + keycode_offset; keymap_idx = 0 }
                if name == "u"         { code = KEY_U           + keycode_offset; keymap_idx = 0 }
                if name == "v"         { code = KEY_V           + keycode_offset; keymap_idx = 0 }
                if name == "w"         { code = KEY_W           + keycode_offset; keymap_idx = 0 }
                if name == "x"         { code = KEY_X           + keycode_offset; keymap_idx = 0 }
                if name == "y"         { code = KEY_Y           + keycode_offset; keymap_idx = 0 }
                if name == "z"         { code = KEY_Z           + keycode_offset; keymap_idx = 0 }
                if name == "A"         { code = KEY_A           + keycode_offset; keymap_idx = 1 }
                if name == "B"         { code = KEY_B           + keycode_offset; keymap_idx = 1 }
                if name == "C"         { code = KEY_C           + keycode_offset; keymap_idx = 1 }
                if name == "D"         { code = KEY_D           + keycode_offset; keymap_idx = 1 }
                if name == "E"         { code = KEY_E           + keycode_offset; keymap_idx = 1 }
                if name == "F"         { code = KEY_F           + keycode_offset; keymap_idx = 1 }
                if name == "G"         { code = KEY_G           + keycode_offset; keymap_idx = 1 }
                if name == "H"         { code = KEY_H           + keycode_offset; keymap_idx = 1 }
                if name == "I"         { code = KEY_I           + keycode_offset; keymap_idx = 1 }
                if name == "J"         { code = KEY_J           + keycode_offset; keymap_idx = 1 }
                if name == "K"         { code = KEY_K           + keycode_offset; keymap_idx = 1 }
                if name == "L"         { code = KEY_L           + keycode_offset; keymap_idx = 1 }
                if name == "M"         { code = KEY_M           + keycode_offset; keymap_idx = 1 }
                if name == "N"         { code = KEY_N           + keycode_offset; keymap_idx = 1 }
                if name == "O"         { code = KEY_O           + keycode_offset; keymap_idx = 1 }
                if name == "P"         { code = KEY_P           + keycode_offset; keymap_idx = 1 }
                if name == "Q"         { code = KEY_Q           + keycode_offset; keymap_idx = 1 }
                if name == "R"         { code = KEY_R           + keycode_offset; keymap_idx = 1 }
                if name == "S"         { code = KEY_S           + keycode_offset; keymap_idx = 1 }
                if name == "T"         { code = KEY_T           + keycode_offset; keymap_idx = 1 }
                if name == "U"         { code = KEY_U           + keycode_offset; keymap_idx = 1 }
                if name == "V"         { code = KEY_V           + keycode_offset; keymap_idx = 1 }
                if name == "W"         { code = KEY_W           + keycode_offset; keymap_idx = 1 }
                if name == "X"         { code = KEY_X           + keycode_offset; keymap_idx = 1 }
                if name == "Y"         { code = KEY_Y           + keycode_offset; keymap_idx = 1 }
                if name == "Z"         { code = KEY_Z           + keycode_offset; keymap_idx = 1 }
                if name == "U0021"     { code = KEY_1           + keycode_offset; keymap_idx = 1 }
                if name == "U0040"     { code = KEY_2           + keycode_offset; keymap_idx = 1 }
                if name == "U0023"     { code = KEY_3           + keycode_offset; keymap_idx = 1 }
                if name == "U0024"     { code = KEY_4           + keycode_offset; keymap_idx = 1 }
                if name == "U0025"     { code = KEY_5           + keycode_offset; keymap_idx = 1 }
                if name == "U005E"     { code = KEY_6           + keycode_offset; keymap_idx = 1 }
                if name == "U0026"     { code = KEY_7           + keycode_offset; keymap_idx = 1 }
                if name == "U002A"     { code = KEY_8           + keycode_offset; keymap_idx = 1 }
                if name == "U0028"     { code = KEY_9           + keycode_offset; keymap_idx = 1 }
                if name == "U0029"     { code = KEY_0           + keycode_offset; keymap_idx = 1 }
                if name == "U002D"     { code = KEY_MINUS       + keycode_offset; keymap_idx = 0 }
                if name == "U005F"     { code = KEY_MINUS       + keycode_offset; keymap_idx = 1 }
                if name == "U003D"     { code = KEY_EQUAL       + keycode_offset; keymap_idx = 0 }
                if name == "U002B"     { code = KEY_EQUAL       + keycode_offset; keymap_idx = 1 }
                if name == "U005B"     { code = KEY_LEFTBRACE   + keycode_offset; keymap_idx = 0 }
                if name == "U007B"     { code = KEY_LEFTBRACE   + keycode_offset; keymap_idx = 1 }
                if name == "U005D"     { code = KEY_RIGHTBRACE  + keycode_offset; keymap_idx = 0 }
                if name == "U007D"     { code = KEY_RIGHTBRACE  + keycode_offset; keymap_idx = 1 }
                if name == "U003B"     { code = KEY_SEMICOLON   + keycode_offset; keymap_idx = 0 }
                if name == "U003A"     { code = KEY_SEMICOLON   + keycode_offset; keymap_idx = 1 }
                if name == "U0027"     { code = KEY_APOSTROPHE  + keycode_offset; keymap_idx = 0 }
                if name == "U0022"     { code = KEY_APOSTROPHE  + keycode_offset; keymap_idx = 1 }
                if name == "U00B4"     { code = KEY_GRAVE       + keycode_offset; keymap_idx = 0 }
                if name == "U007E"     { code = KEY_GRAVE       + keycode_offset; keymap_idx = 1 }
                if name == "U005C"     { code = KEY_BACKSLASH   + keycode_offset; keymap_idx = 0 }
                if name == "U007C"     { code = KEY_BACKSLASH   + keycode_offset; keymap_idx = 1 }
                if name == "U002C"     { code = KEY_COMMA       + keycode_offset; keymap_idx = 0 }
                if name == "U003C"     { code = KEY_COMMA       + keycode_offset; keymap_idx = 1 }
                if name == "U002E"     { code = KEY_DOT         + keycode_offset; keymap_idx = 0 }
                if name == "U003E"     { code = KEY_DOT         + keycode_offset; keymap_idx = 1 }
                if name == "U002F"     { code = KEY_SLASH       + keycode_offset; keymap_idx = 0 }
                if name == "U003F"     { code = KEY_SLASH       + keycode_offset; keymap_idx = 1 }
                if name == "U0020"     { code = KEY_SPACE       + keycode_offset; keymap_idx = 0 }
                if name == "BackSpace" { code = KEY_BACKSPACE   + keycode_offset; keymap_idx = 0 }
                if name == "Delete"    { code = KEY_DELETE      + keycode_offset; keymap_idx = 0 }
                if name == "Down"      { code = KEY_DOWN        + keycode_offset; keymap_idx = 0 }
                if name == "Left"      { code = KEY_LEFT        + keycode_offset; keymap_idx = 0 }
                if name == "Right"     { code = KEY_RIGHT       + keycode_offset; keymap_idx = 0 }
                if name == "Up"        { code = KEY_UP          + keycode_offset; keymap_idx = 0 }
                if name == "End"       { code = KEY_END         + keycode_offset; keymap_idx = 0 }
                if name == "Escape"    { code = KEY_ESC         + keycode_offset; keymap_idx = 0 }
                if name == "F1"        { code = KEY_F1          + keycode_offset; keymap_idx = 0 }
                if name == "F2"        { code = KEY_F2          + keycode_offset; keymap_idx = 0 }
                if name == "F3"        { code = KEY_F3          + keycode_offset; keymap_idx = 0 }
                if name == "F4"        { code = KEY_F4          + keycode_offset; keymap_idx = 0 }
                if name == "F5"        { code = KEY_F5          + keycode_offset; keymap_idx = 0 }
                if name == "F6"        { code = KEY_F6          + keycode_offset; keymap_idx = 0 }
                if name == "F7"        { code = KEY_F7          + keycode_offset; keymap_idx = 0 }
                if name == "F8"        { code = KEY_F8          + keycode_offset; keymap_idx = 0 }
                if name == "F9"        { code = KEY_F9          + keycode_offset; keymap_idx = 0 }
                if name == "F10"       { code = KEY_F10         + keycode_offset; keymap_idx = 0 }
                if name == "F11"       { code = KEY_F11         + keycode_offset; keymap_idx = 0 }
                if name == "F12"       { code = KEY_F12         + keycode_offset; keymap_idx = 0 }
                if name == "Home"      { code = KEY_HOME        + keycode_offset; keymap_idx = 0 }
                if name == "Insert"    { code = KEY_INSERT      + keycode_offset; keymap_idx = 0 }
                if name == "Menu"      { code = KEY_MENU        + keycode_offset; keymap_idx = 0 }
                if name == "Page_Down" { code = KEY_PAGEDOWN    + keycode_offset; keymap_idx = 0 }
                if name == "Page_Up"   { code = KEY_PAGEUP      + keycode_offset; keymap_idx = 0 }
                if name == "Pause"     { code = KEY_PAUSE       + keycode_offset; keymap_idx = 0 }
                if name == "Return"    { code = KEY_ENTER       + keycode_offset; keymap_idx = 0 }
                if name == "Tab"       { code = KEY_TAB         + keycode_offset; keymap_idx = 0 }
                (String::from(name), KeyCode { code, keymap_idx })
            }),
    );
    // Workaround: BackSpace does not work with `tools/entry.py` (made with GTK3),
    // if the keymap with BackSpace does not contain any other keycodes.
    // This should only happen for the emoji-layout or incomplete custom-layouts,
    // because the layout-tests for normal layouts check for the presence of a button for Return.
    // This does add an "Unknown"-keycode, if necessary, to let BackSpace work anyway.
    if !HashMap::contains_key(&keycode_map, &"Return".to_string()) {
        HashMap::insert(&mut keycode_map,
                        "Unknown".to_string(),
                        KeyCode { code: KEY_UNKNOWN + keycode_offset, keymap_idx: 0 }
        );
    }
    keycode_map
}

#[derive(Debug)]
pub enum FormattingError {
    Utf(FromUtf8Error),
    Format(io::Error),
}

impl fmt::Display for FormattingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattingError::Utf(e) => write!(f, "UTF: {}", e),
            FormattingError::Format(e) => write!(f, "Format: {}", e),
        }
    }
}

impl From<io::Error> for FormattingError {
    fn from(e: io::Error) -> Self {
        FormattingError::Format(e)
    }
}

/// Index is the key code, String is the occupant.
/// Starts all empty.
/// https://gitlab.freedesktop.org/xorg/xserver/-/issues/260
type SingleKeyMap = [Option<String>; 256];

fn single_key_map_new() -> SingleKeyMap {
    // Why can't we just initialize arrays without tricks -_- ?
    // Inspired by
    // https://www.reddit.com/r/rust/comments/5n7bh1/how_to_create_an_array_of_a_type_with_clone_but/
    let mut array = mem::MaybeUninit::<SingleKeyMap>::uninit();

    unsafe {
        let arref = &mut *array.as_mut_ptr();
        for element in arref.iter_mut() {
            ptr::write(element, None);
        }

        array.assume_init()
    }
}

pub fn generate_keymaps(symbolmap: HashMap::<String, KeyCode>)
    -> Result<Vec<String>, FormattingError>
{
    let mut bins: Vec<SingleKeyMap> = Vec::new();
    
    for (name, KeyCode { code, keymap_idx }) in symbolmap.into_iter() {
        if keymap_idx >= bins.len() {
            bins.resize_with(
                keymap_idx + 1,
                || single_key_map_new(),
            );
        }
        bins[keymap_idx][code as usize] = Some(name);
    }

    let mut out = Vec::new();
    for bin in bins {
        out.push(generate_keymap(&bin)?);
    }
    Ok(out)
}

/// Generates a de-facto single level keymap.
/// Key codes must not repeat and must remain between 9 and 255.
fn generate_keymap(
    symbolmap: &SingleKeyMap,
) -> Result<String, FormattingError> {
    let mut buf: Vec<u8> = Vec::new();
    writeln!(
        buf,
        "xkb_keymap {{

    xkb_keycodes \"squeekboard\" {{
        minimum = 8;
        maximum = 255;"
    )?;

    let pairs: Vec<(&String, usize)> = symbolmap.iter()
        // Attach a key code to each cell.
        .enumerate()
        // Get rid of empty keycodes.
        .filter_map(|(code, name)| name.as_ref().map(|n| (n, code)))
        .collect();
    
    // Xorg can only consume up to 255 keys, so this may not work in Xwayland.
    // Two possible solutions:
    // - use levels to cram multiple characters into one key
    // - swap layouts on key presses
    for (_name, keycode) in &pairs {
        write!(
            buf,
            "
        <I{}> = {0};",
            keycode,
        )?;
    }

    writeln!(
        buf,
        "
        indicator 1 = \"Caps Lock\"; // Xwayland won't accept without it.
    }};
    
    xkb_symbols \"squeekboard\" {{
"
    )?;
    
    for (name, keycode) in pairs {
        write!(
            buf,
            "
key <I{}> {{ [ {} ] }};",
            keycode,
            name,
        )?;
    }

    writeln!(
        buf,
        "
    }};

    xkb_types \"squeekboard\" {{
        virtual_modifiers Squeekboard; // No modifiers! Needed for Xorg for some reason.
    
        // Those names are needed for Xwayland.
        type \"ONE_LEVEL\" {{
            modifiers= none;
            level_name[Level1]= \"Any\";
        }};
        type \"TWO_LEVEL\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"ALPHABETIC\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"KEYPAD\" {{
            level_name[Level1]= \"Base\";
        }};
        type \"SHIFT+ALT\" {{
            level_name[Level1]= \"Base\";
        }};

    }};

    xkb_compatibility \"squeekboard\" {{
        // Needed for Xwayland again.
        interpret Any+AnyOf(all) {{
            action= SetMods(modifiers=modMapMods,clearLocks);
        }};
    }};
}};"
    )?;
    
    //println!("{}", String::from_utf8(buf.clone()).unwrap());
    String::from_utf8(buf).map_err(FormattingError::Utf)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use xkbcommon::xkb;

    #[test]
    fn test_keymap_single_resolve() {
        let mut key_map = single_key_map_new();
        key_map[9] = Some("a".into());
        key_map[10] = Some("c".into());

        let keymap_str = generate_keymap(&key_map).unwrap();

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

        let keymap = xkb::Keymap::new_from_string(
            &context,
            keymap_str.clone(),
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        ).expect("Failed to create keymap");

        let state = xkb::State::new(&keymap);

        assert_eq!(state.key_get_one_sym(9u32.into()), xkb::keysyms::KEY_a.into());
        assert_eq!(state.key_get_one_sym(10u32.into()), xkb::keysyms::KEY_c.into());
    }

    #[test]
    fn test_keymap_second_resolve() {
        let keymaps = generate_keymaps(hashmap!(
            "a".into() => KeyCode { keymap_idx: 1, code: 9 },
        )).unwrap();

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);

        let keymap = xkb::Keymap::new_from_string(
            &context,
            keymaps[1].clone(), // this index is part of the test
            xkb::KEYMAP_FORMAT_TEXT_V1,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        ).expect("Failed to create keymap");

        let state = xkb::State::new(&keymap);

        assert_eq!(state.key_get_one_sym(9u32.into()), xkb::keysyms::KEY_a.into());
    }

    #[test]
    fn test_symbolmap_overflow() {
        // Use Unicode encoding for being able to use in xkb keymaps.
        let keynames = (0..258).map(|num| format!("U{:04X}", 0x1000 + num));
        let keycodes = generate_keycodes(keynames);

        assert_eq!(keycodes.into_iter().filter(|(_name, keycode)| keycode.code > 255).count(), 0);
    }
}
