use bevy::{app::*, math::IVec2, prelude::*, window::*};
use ron::de::from_reader;
use ron::ser::{to_writer_pretty, PrettyConfig};
use std::fs::File;
use std::path::PathBuf;

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
            .add_system(resize_notification.system())
            .add_system(moved_notification.system());
    }
}

fn config_path() -> String {
    let path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "assets", "plugins"]
        .iter()
        .collect();
    path.into_os_string().into_string().unwrap()
}

fn config_file() -> String {
    let path: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "assets",
        "plugins",
        "window.config.ron",
    ]
    .iter()
    .collect();
    path.into_os_string().into_string().unwrap()
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    // Load Config or default
    let config = match File::open(config_file()) {
        Ok(x) => from_reader(x).unwrap(),
        Err(e) => {
            println!("Failed to load config: {}", e);
            WindowConfig {
                height: 800.,
                width: 1200.,
                position: IVec2::new(0, 0),
            }
        }
    };

    let window = windows.get_primary_mut().unwrap();

    window.set_position(config.position);
    window.set_resolution(config.width, config.height);
    commands.insert_resource(config);
}

fn resize_notification(resize_event: Res<Events<WindowResized>>, mut windows: ResMut<Windows>) {
    let mut reader = resize_event.get_reader();
    for _ in reader.iter(&resize_event) {
        let window = windows.get_primary_mut().unwrap();
        save_change(window);
    }
}

fn moved_notification(move_event: Res<Events<WindowMoved>>, mut windows: ResMut<Windows>) {
    let mut reader = move_event.get_reader();
    for _ in reader.iter(&move_event) {
        let window = windows.get_primary_mut().unwrap();
        save_change(window);
    }
}

fn save_change(window: &mut Window) {
    let value = WindowConfig {
        width: window.width(),
        height: window.height(),
        position: window.position().unwrap(),
    };

    let pretty = PrettyConfig::new()
        .with_depth_limit(2)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);

    let _ = std::fs::create_dir_all(config_path());
    let f = File::create(config_file()).expect("Failed opening file");

    match to_writer_pretty(f, &value, pretty) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to write config: {}", e);
        }
    };
}
