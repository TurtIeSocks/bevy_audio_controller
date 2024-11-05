use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

#[derive(Component, Default, AudioChannel)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
struct SfxChannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, setup)
        .add_systems(Update, (play_sfx,))
        .add_systems(PostUpdate, (do_something_with_sfx, do_something_with_fire))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn play_sfx(mut commands: Commands) {
    commands.spawn((AudioFiles::FireOGG, PlaybackSettings::DESPAWN));
    commands.spawn((AudioFiles::SprayOGG, PlaybackSettings::DESPAWN, SfxChannel));
}

// This system will run after the `AudioSink` components have been added to any entities on the `SfxChannel`
fn do_something_with_sfx(
    sfx_query: Query<(Entity, &Name, &AudioSink), (Added<AudioSink>, With<SfxChannel>)>,
) {
    for (entity, name, sink) in sfx_query.iter() {
        sink.set_volume(0.75);
        info!(
            "Sfx: {} ({}) is playing at volume {}",
            name,
            entity,
            sink.volume()
        );
    }
}

// This system will run after the `AudioSink` components have been added to any `FireOGG` entities
fn do_something_with_fire(
    sfx_query: Query<(Entity, &Name, &AudioSink), (Added<AudioSink>, With<FireOGG>)>,
) {
    for (entity, name, sink) in sfx_query.iter() {
        sink.set_speed(1.25);
        info!(
            "Fire: {} ({}) is playing at speed {}",
            name,
            entity,
            sink.speed()
        );
    }
}
