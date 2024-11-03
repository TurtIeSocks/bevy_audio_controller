use bevy::{
    input::common_conditions::{input_just_pressed, input_toggle_active},
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

mod helpers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                wait_mode.run_if(input_toggle_active(true, KeyCode::Space)),
                immediate_mode.run_if(input_toggle_active(false, KeyCode::Space)),
                helpers::despawn_on_change.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(helpers::get_container())
        .with_children(|parent| {
            parent.spawn(helpers::get_text(
                "Press SPACE to toggle between\n \"DelayMode::Wait\" and \"DelayMode::Immediate\"",
            ));
        });
}

fn wait_mode(mut commands: Commands) {
    // By default, `DelayMode::Wait` will be used if it's omitted
    commands.spawn((AudioFiles::FireOGG, PlaybackSettings::DESPAWN));
    commands.spawn((
        AudioFiles::SprayOGG,
        PlaybackSettings::DESPAWN,
        DelayMode::Wait,
    ));
}

fn immediate_mode(mut commands: Commands) {
    commands.spawn((
        AudioFiles::FireOGG,
        PlaybackSettings::DESPAWN,
        DelayMode::Immediate,
    ));
}
