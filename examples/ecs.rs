use bevy::{
    input::common_conditions::{input_just_pressed, input_toggle_active},
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

mod helpers;

#[derive(Component, Default, AudioChannel)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
struct FireChannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<FireChannel>()
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

fn setup(mut commands: Commands, mut settings: ResMut<ChannelSettings<GlobalChannel>>) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(helpers::get_container())
        .with_children(|parent| {
            parent.spawn(helpers::get_text(
                "Press SPACE to toggle between\n \"DelayMode::Wait\" and \"DelayMode::Immediate\"",
                40.0,
            ));
        });

    settings.set_default_settings(PlaybackSettings::DESPAWN);
}

fn wait_mode(mut commands: Commands) {
    commands.spawn((
        AudioFiles::FireOGG,
        FireChannel,
        PlaybackSettings::DESPAWN,
        // By default, `DelayMode::Wait` will be used if it's omitted
        // DelayMode::Wait,
    ));
    commands.spawn((
        AudioFiles::SprayOGG,
        DelayMode::Milliseconds(500),
        // We can override playback settings here but we've set it above for all so it's unnecessary
        // PlaybackSettings::DESPAWN,
    ));
}

fn immediate_mode(mut commands: Commands) {
    commands.spawn((AudioFiles::FireOGG, DelayMode::Immediate));
}
