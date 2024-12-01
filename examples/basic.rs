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
                play_with_plugin.run_if(input_toggle_active(true, KeyCode::Space)),
                play_without_plugin.run_if(input_toggle_active(false, KeyCode::Space)),
                helpers::despawn_on_change.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(helpers::get_container())
        .with_children(|parent| {
            parent.spawn(helpers::get_text(
                "Press SPACE to toggle between\nplugin and non-plugin audio",
                40.0,
            ));
        });
}

fn play_with_plugin(mut sfx_play_ew: EventWriter<GlobalPlayEvent>) {
    let event = GlobalPlayEvent::new(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN);
    sfx_play_ew.send(event);
    // You can send events using the enum values or a string
    // sfx_play_ew.send(GlobalPlayEvent::new("fire.ogg".into()));
}

fn play_without_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PlaybackSettings::DESPAWN,
        AudioPlayer::<AudioSource>(asset_server.load("fire.ogg")),
    ));
}
