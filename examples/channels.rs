use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

#[derive(Component, Default, AudioChannel, Reflect)]
struct MusicChannel;

#[derive(Component, Default, AudioChannel, Reflect)]
struct SfxChannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<MusicChannel>()
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, setup)
        .add_systems(Update, play_sfx)
        .run();
}

fn setup(mut commands: Commands, mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    commands.spawn(Camera2dBundle::default());
    let event =
        MusicChannel::play_event("background.ogg".into()).with_settings(PlaybackSettings::LOOP);
    ew.send(event);
}

fn play_sfx(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    let event =
        SfxChannel::play_event(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN);
    ew.send(event);
}
