use bevy::{app::*, math::IVec2, prelude::*, window::*};
use ron::de::from_reader;
use ron::ser::{to_writer_pretty, PrettyConfig};
use std::fs::File;

#[derive(Default, serde::Deserialize, serde::Serialize, Debug)]
struct WindowConfig {
    width: f32,
    height: f32,
    position: IVec2,
}

pub struct WindowConfigPlugin;

impl Plugin for WindowConfigPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(resize_notificator.system())
            .add_system(moved_notification.system());
    }
}

fn config_path() -> String {
    format!("{}/assets/plugins/window.config.ron", env!("CARGO_MANIFEST_DIR"))
}

fn setup(mut windows: ResMut<Windows>) {
    let f = File::open(config_path()).expect("Failed opening file");

    let config: WindowConfig = from_reader(f).unwrap();

    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(config.width, config.height);
    window.set_position(config.position);
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>, mut windows: ResMut<Windows>) {
    let mut reader = resize_event.get_reader();
     for e in reader.iter(&resize_event) {
         //println!("width = {} height = {}", e.width, e.height);
     }
    let window = windows.get_primary_mut().unwrap();
    save_change(window);
}

fn moved_notification(move_event: Res<Events<WindowMoved>>, mut windows: ResMut<Windows>) {
    let mut reader = move_event.get_reader();
     for e in reader.iter(&move_event) {
    //     println!("position = {}", e.position);
     }

    let window = windows.get_primary_mut().unwrap();
    save_change(window);
}

fn save_change(window: &mut Window) {
    let f = File::create(config_path()).expect("Failed opening file");
    let value = WindowConfig {
        width: window.width(),
        height: window.height(),
        position: window.position().unwrap(),
    };

    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);

    let write = match to_writer_pretty(f, &value, pretty) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };
}
