#![allow(non_snake_case)]

use bevy::prelude::*;
use bevy::text::FontSmoothing;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const CLICKED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn MAIN_MENU_NODE() -> Node {
	Node {
		flex_direction: FlexDirection::Column,
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		width: Val::Px(300.0),
		height: Val::Px(120.0),
		border: UiRect::all(Val::Px(5.0)),
		..default()
	}
}

pub fn MAIN_MENU_TITLE_NODE() -> Node {
	Node {
		flex_direction: FlexDirection::Row,
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		width: Val::Px(300.0),
		height: Val::Px(120.0),
		border: UiRect::all(Val::Px(5.0)),
		..default()
	}
}

pub fn MAIN_MENU_BUTTON_NODE() -> Node {
	Node {
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		width: Val::Px(400.0),
		height: Val::Px(120.0),
		margin: UiRect::all(Val::Px(8.0)),
		..DEFAULT_BUTTON_NODE()
	}
}

pub fn DEFAULT_BUTTON_NODE() -> Node {
	Node {
		justify_content: JustifyContent::Center,
		align_items: AlignItems::Center,
		width: Val::Px(400.0),
		height: Val::Px(80.0),
		..default()
	}
}

// pub const TITLE_STYLE: Style = {
// 	let mut style = Style::DEFAULT;
// 	style.flex_direction = FlexDirection::Row;
// 	style.justify_content = JustifyContent::Center;
// 	style.align_items = AlignItems::Center;
// 	style.width = Val::Px(300.0);
// 	style.height = Val::Px(120.0);
// 	style
// };
//
// pub const TITLE_IMAGE_STYLE: Style = {
// 	let mut style = Style::DEFAULT;
// 	style.width = Val::Px(64.0);
// 	style.height = Val::Px(64.0);
// 	style.margin = UiRect::all(Val::Px(8.0));
// 	style
// };

// #[allow(clippy::module_name_repetitions)]
// pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
// 	TextStyle {
// 		font: asset_server.load("fonts/Roboto-Bold.ttf"),
// 		font_size: 64.0,
// 		color: Color::WHITE,
// 	}
// }

#[allow(clippy::module_name_repetitions)]
pub fn BUTTON_TEXT_FONT(asset_server: &Res<AssetServer>) -> TextFont {
	TextFont {
		font: asset_server.load("fonts/Roboto-Bold.ttf"),
		font_size: 32.0,
		font_smoothing: FontSmoothing::AntiAliased,
	}
}

pub fn LOGO_NODE(asset_server: &Res<AssetServer>) -> (ImageNode, Node) {
	(
		ImageNode {
			image: asset_server.load("logo.png").into(),
			..default()
		},
		Node {
			width: Val::Px(64.0),
			height: Val::Px(64.0),
			margin: UiRect::all(Val::Px(8.0)),
			..default()
		},
	)
}
