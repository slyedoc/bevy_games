use bevy::prelude::*;
use bevy_inspector_egui::*;

#[derive(Inspectable)]
pub enum TextColor {
    White,
    Green,
    Blue,
}

#[derive(Inspectable)]
pub struct Data {
    #[inspectable(min = 10.0, max = 70.0)]
    font_size: f32,
    text: String,
    show_square: bool,
    text_color: TextColor,
    color: Color,
    #[inspectable(visual, min = Vec2::new(-200., -200.), max = Vec2::new(200., 200.))]
    position: Vec2,
    list: Vec<f32>,
    #[inspectable(replacement = String::default as fn() -> _)]
    option: Option<String>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            font_size: 50.0,
            text: "Hello World!".to_string(),
            show_square: true,
            text_color: TextColor::White,
            color: Color::BLUE,
            position: Vec2::default(),
            list: vec![0.0],
            option: None,
        }
    }
}



pub fn text_update_system(data: Res<Data>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        let text = &mut text.sections[0];
        text.value = data.text.clone();
        text.style.font_size = data.font_size;
        text.style.color = match &data.text_color {
            TextColor::White => Color::WHITE,
            TextColor::Green => Color::GREEN,
            TextColor::Blue => Color::BLUE,
        };
    }
}

pub fn shape_update_system(
    data: Res<Data>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Handle<ColorMaterial>, &mut Transform)>,
) {
    for (color, mut transfrom) in query.iter_mut() {
        let material = materials.get_mut(color).unwrap();
        material.color = data.color;

        if !data.show_square {
            transfrom.translation.x = 1000000.0;
        } else {
            transfrom.translation.x = data.position.x;
            transfrom.translation.y = data.position.y;
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let font_handle = asset_server.load("fonts/noto/NotoMono-Regular.ttf");

    let color = materials.add(Color::BLUE.into());

    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            ..Default::default()
        },
        text: Text::with_section(
            "",
            TextStyle {
                font_size: 50.0,
                font: font_handle,
                ..Default::default()
            },
            Default::default(),
        ),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        material: color,
        sprite: Sprite {
            size: Vec2::new(40.0, 40.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::default()),
        ..Default::default()
    });
}