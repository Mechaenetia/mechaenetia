use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const CLICKED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const MAIN_MENU_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.flex_direction = FlexDirection::Column;
	style.justify_content = JustifyContent::Center;
	style.align_items = AlignItems::Center;
	style.width = Val::Percent(100.0);
	style.height = Val::Percent(100.0);
	style.row_gap = Val::Px(8.0);
	style.column_gap = Val::Px(8.0);
	style
};

pub const BUTTON_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.justify_content = JustifyContent::Center;
	style.align_items = AlignItems::Center;
	style.width = Val::Px(400.0);
	style.height = Val::Px(80.0);
	style
};

pub const TITLE_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.flex_direction = FlexDirection::Row;
	style.justify_content = JustifyContent::Center;
	style.align_items = AlignItems::Center;
	style.width = Val::Px(300.0);
	style.height = Val::Px(120.0);
	style
};

pub const TITLE_IMAGE_STYLE: Style = {
	let mut style = Style::DEFAULT;
	style.width = Val::Px(64.0);
	style.height = Val::Px(64.0);
	style.margin = UiRect::all(Val::Px(8.0));
	style
};

#[allow(clippy::module_name_repetitions)]
pub fn get_title_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Roboto-Bold.ttf"),
		font_size: 64.0,
		color: Color::WHITE,
	}
}

#[allow(clippy::module_name_repetitions)]
pub fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
	TextStyle {
		font: asset_server.load("fonts/Roboto-Bold.ttf"),
		font_size: 32.0,
		color: Color::WHITE,
	}
}
