use bevy::input::keyboard::KeyCode as BKC;
use bevy::input::keyboard::KeyboardInput as BKI;
use bevy::input::ElementState as BES;
use crossterm::event::KeyCode as CKC;
use crossterm::event::KeyEvent as CKE;
use crossterm::event::KeyModifiers as CKM;
use tracing::*;

macro_rules! kp {
	($key:expr, $scan:expr) => {
		BKI {
			scan_code: $scan,
			key_code: Some($key),
			state: BES::Pressed,
		}
	};
}

macro_rules! kr {
	($key:expr, $scan:expr) => {
		BKI {
			scan_code: $scan,
			key_code: Some($key),
			state: BES::Released,
		}
	};
}

pub fn to_bevy_iterator(key: CKE) -> impl Iterator<Item = BKI> {
	use BKC::*;
	let key_code: &[BKI] = match key.code {
		CKC::Null => &[BKI {
			scan_code: 0,
			key_code: None,
			state: BES::Pressed,
		}],
		CKC::Esc => &[kp!(Escape, 27)],
		CKC::Backspace => &[kp!(Back, 8)],
		CKC::Enter => &[kp!(Return, 10)],
		CKC::Left => &[kp!(Left, 256)],
		CKC::Right => &[kp!(Right, 257)],
		CKC::Up => &[kp!(Up, 258)],
		CKC::Down => &[kp!(Down, 259)],
		CKC::Home => &[kp!(Home, 260)],
		CKC::End => &[kp!(End, 261)],
		CKC::PageUp => &[kp!(PageUp, 262)],
		CKC::PageDown => &[kp!(PageDown, 263)],
		CKC::Tab => &[kp!(Tab, 9)],
		CKC::BackTab => &[kp!(LShift, 0), kp!(Tab, 9)],
		CKC::Delete => &[kp!(Delete, 127)],
		CKC::Insert => &[kp!(Insert, 264)],
		CKC::F(id) => match id {
			1 => &[kp!(F1, 266)],
			2 => &[kp!(F2, 267)],
			3 => &[kp!(F3, 268)],
			4 => &[kp!(F4, 269)],
			5 => &[kp!(F5, 270)],
			6 => &[kp!(F6, 271)],
			7 => &[kp!(F7, 272)],
			8 => &[kp!(F8, 273)],
			9 => &[kp!(F9, 274)],
			10 => &[kp!(F10, 275)],
			11 => &[kp!(F11, 276)],
			12 => &[kp!(F12, 277)],
			13 => &[kp!(F13, 278)],
			14 => &[kp!(F14, 279)],
			15 => &[kp!(F15, 280)],
			16 => &[kp!(F16, 281)],
			17 => &[kp!(F17, 282)],
			18 => &[kp!(F18, 283)],
			19 => &[kp!(F19, 284)],
			20 => &[kp!(F20, 285)],
			21 => &[kp!(F21, 286)],
			22 => &[kp!(F22, 287)],
			23 => &[kp!(F23, 288)],
			24 => &[kp!(F24, 289)],
			_ => {
				warn!("unhandled F# key ID: {}", id);
				&[]
			}
		},
		// For a-z the upper-case has the modifier set, but not for most others, its odd...
		CKC::Char(c) => match c {
			'A' => &[kp!(RShift, 0), kp!(A, 65)],
			'B' => &[kp!(RShift, 0), kp!(B, 66)],
			'C' => &[kp!(RShift, 0), kp!(C, 67)],
			'D' => &[kp!(RShift, 0), kp!(D, 68)],
			'E' => &[kp!(RShift, 0), kp!(E, 69)],
			'F' => &[kp!(RShift, 0), kp!(F, 70)],
			'G' => &[kp!(RShift, 0), kp!(G, 71)],
			'H' => &[kp!(RShift, 0), kp!(H, 72)],
			'I' => &[kp!(RShift, 0), kp!(I, 73)],
			'J' => &[kp!(RShift, 0), kp!(J, 74)],
			'K' => &[kp!(RShift, 0), kp!(K, 75)],
			'L' => &[kp!(RShift, 0), kp!(L, 76)],
			'M' => &[kp!(RShift, 0), kp!(M, 77)],
			'N' => &[kp!(RShift, 0), kp!(N, 78)],
			'O' => &[kp!(RShift, 0), kp!(O, 79)],
			'P' => &[kp!(RShift, 0), kp!(P, 80)],
			'Q' => &[kp!(RShift, 0), kp!(Q, 81)],
			'R' => &[kp!(RShift, 0), kp!(R, 82)],
			'S' => &[kp!(RShift, 0), kp!(S, 83)],
			'T' => &[kp!(RShift, 0), kp!(T, 84)],
			'U' => &[kp!(RShift, 0), kp!(U, 85)],
			'V' => &[kp!(RShift, 0), kp!(V, 86)],
			'W' => &[kp!(RShift, 0), kp!(W, 87)],
			'X' => &[kp!(RShift, 0), kp!(X, 88)],
			'Y' => &[kp!(RShift, 0), kp!(Y, 89)],
			'Z' => &[kp!(RShift, 0), kp!(Z, 90)],
			'a' => &[kp!(A, 97)],
			'b' => &[kp!(B, 98)],
			'c' => &[kp!(C, 99)],
			'd' => &[kp!(D, 100)],
			'e' => &[kp!(E, 101)],
			'f' => &[kp!(F, 102)],
			'g' => &[kp!(G, 103)],
			'h' => &[kp!(H, 104)],
			'i' => &[kp!(I, 105)],
			'j' => &[kp!(J, 106)],
			'k' => &[kp!(K, 107)],
			'l' => &[kp!(L, 108)],
			'm' => &[kp!(M, 109)],
			'n' => &[kp!(N, 110)],
			'o' => &[kp!(O, 111)],
			'p' => &[kp!(P, 112)],
			'q' => &[kp!(Q, 113)],
			'r' => &[kp!(R, 114)],
			's' => &[kp!(S, 115)],
			't' => &[kp!(T, 116)],
			'u' => &[kp!(U, 117)],
			'v' => &[kp!(V, 118)],
			'w' => &[kp!(W, 119)],
			'x' => &[kp!(X, 120)],
			'y' => &[kp!(Y, 121)],
			'z' => &[kp!(Z, 122)],
			'0' => &[kp!(Key0, 48)],
			'1' => &[kp!(Key1, 49)],
			'2' => &[kp!(Key2, 50)],
			'3' => &[kp!(Key3, 51)],
			'4' => &[kp!(Key4, 52)],
			'5' => &[kp!(Key5, 53)],
			'6' => &[kp!(Key6, 54)],
			'7' => &[kp!(Key7, 55)],
			'8' => &[kp!(Key8, 56)],
			'9' => &[kp!(Key9, 57)],
			' ' => &[kp!(Space, 32)],
			'`' => &[kp!(Grave, 96)],
			'-' => &[kp!(Minus, 45)],
			'=' => &[kp!(Equals, 61)],
			'[' => &[kp!(LBracket, 91)],
			']' => &[kp!(RBracket, 93)],
			'\\' => &[kp!(Backslash, 124)],
			';' => &[kp!(Semicolon, 59)],
			'\'' => &[kp!(Apostrophe, 39)],
			'"' => &[kp!(RShift, 0), kp!(Apostrophe, 34)],
			':' => &[kp!(Colon, 58)],
			',' => &[kp!(Comma, 44)],
			'.' => &[kp!(Period, 46)],
			'/' => &[kp!(Slash, 47)],
			'_' => &[kp!(Underline, 95)],
			'+' => &[kp!(Plus, 43)],
			'{' => &[kp!(RShift, 0), kp!(LBracket, 123)],
			'}' => &[kp!(RShift, 0), kp!(RBracket, 125)],
			'|' => &[kp!(RShift, 0), kp!(Backslash, 124)],
			'<' => &[kp!(LBracket, 60)],
			'>' => &[kp!(RBracket, 62)],
			'?' => &[kp!(Slash, 63)],
			'~' => &[kp!(RShift, 0), kp!(Grave, 126)],
			'!' => &[kp!(RShift, 0), kp!(Key1, 33)],
			'@' => &[kp!(At, 64)],
			'#' => &[kp!(RShift, 0), kp!(Key3, 35)],
			'$' => &[kp!(RShift, 0), kp!(Key4, 36)],
			'%' => &[kp!(RShift, 0), kp!(Key5, 37)],
			'^' => &[kp!(Caret, 94)],
			'&' => &[kp!(RShift, 0), kp!(Key7, 38)],
			'*' => &[kp!(Asterisk, 42)],
			'(' => &[kp!(RShift, 0), kp!(Key9, 40)],
			')' => &[kp!(RShift, 0), kp!(Key0, 41)],
			_ => {
				error!(
					"unhandled keyboard char code in TUI '{}' with modifiers: {:?}",
					c, key.modifiers
				);
				&[]
			}
		},
	};
	let modifier_code: &[BKC] = {
		if key.modifiers.contains(CKM::SHIFT) {
			if key.modifiers.contains(CKM::CONTROL) {
				if key.modifiers.contains(CKM::ALT) {
					&[BKC::LShift, BKC::LControl, BKC::LAlt]
				} else {
					&[BKC::LShift, BKC::LControl]
				}
			} else {
				if key.modifiers.contains(CKM::ALT) {
					&[BKC::LShift, BKC::LAlt]
				} else {
					&[BKC::LShift]
				}
			}
		} else {
			if key.modifiers.contains(CKM::CONTROL) {
				if key.modifiers.contains(CKM::ALT) {
					&[BKC::LControl, BKC::LAlt]
				} else {
					&[BKC::LControl]
				}
			} else {
				if key.modifiers.contains(CKM::ALT) {
					&[BKC::LAlt]
				} else {
					&[]
				}
			}
		}
	};

	let modifier_iter = modifier_code.iter().map(|c| kp!(*c, 0));
	let keys_iter = key_code.iter();
	let keys_rev_iter = key_code.iter().rev().map(|i| BKI {
		state: BES::Released,
		..*i
	});
	let modifier_rev_iter = modifier_code.iter().rev().map(|c| kr!(*c, 0));

	modifier_iter
		.chain(keys_iter.cloned())
		.chain(keys_rev_iter)
		.chain(modifier_rev_iter)
}
