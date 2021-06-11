pub struct BoardPlugin;

impl Plugin for MountainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(MountainTimer(Timer::from_seconds(3.0, true)))
            .add_system(mountain_spawn_system.system());
    }
}

fn mountain_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut mountain_timer: ResMut<MountainTimer>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    mountain_timer.0.tick(time.delta());
    if mountain_timer.0.finished() {
        // TODO: find correct way to copy handle for "use after moved" error
        let mountain_texture: Handle<Texture> = asset_server.load("mountain.png");
        let mountain_texture2: Handle<Texture> = asset_server.load("mountain.png");
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(1920.0 * 0.5 + 30.0 * 43.0, -1280.0 * 0.5, 0.2),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(3.0),
                },
                material: materials.add(ColorMaterial::modulated_texture(
                    mountain_texture,
                    Color::rgb(0.36, 0.36, 0.36),
                )),
                ..Default::default()
            })
            .insert(OffsceenDeletion)
            .insert(Velocity(Vec2::new(-200.0, 0.0)));

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(1920.0 * 0.5 + 30.0 * 43.0, -1280.0 * 0.5 - 100.0, 0.3),
                    rotation: Quat::IDENTITY,
                    scale: Vec3::splat(3.0),
                },
                material: materials.add(ColorMaterial::modulated_texture(
                    mountain_texture2,
                    Color::rgb(0.26, 0.26, 0.26),
                )),
                ..Default::default()
            })
            .insert(OffsceenDeletion)
            .insert(Velocity(Vec2::new(-400.0, 0.0)));
    }
}
