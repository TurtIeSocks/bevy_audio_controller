use bevy::{log::LogPlugin, prelude::*};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

mod helpers;

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
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, (setup,))
        .add_systems(Update, (play_sfx,))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands
        .spawn(helpers::get_container())
        .with_children(|parent| {
            parent.spawn(helpers::get_text("`fire.ogg` has a -500ms delay", 40.0));
            parent.spawn(helpers::get_text(
                "This will set the next one to play (length of `fire.ogg` - 500ms) after each spawn",
                20.0,
            ));
            parent.spawn(helpers::get_text(
                "Resulting in an overlap",
                20.0,
            ));
            parent.spawn(helpers::get_text("", 0.0));
            parent.spawn(helpers::get_text(
                "`spray.ogg` will play with a 200% delay",
                40.0,
            ));
            parent.spawn(helpers::get_text(
                "This will set the next one to play (length of spray.ogg track * 2.0) after each spawn",
                20.0,
            ));
            parent.spawn(helpers::get_text(
                "Resulting in a gap between plays",
                20.0,
            ));
        });
}

fn play_sfx(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    // Plays the spray sound every 200% * duration of `spray.ogg` from the time that the sound is spawned
    // Resulting in a gap between plays
    ew.send(
        SfxChannel::play_event(AudioFiles::SprayOGG)
            .with_settings(PlaybackSettings::DESPAWN)
            .with_delay_mode(DelayMode::Percent(200)),
    );
    // Plays the fire sound duration of `fire.ogg` - 500 milliseconds from the time that the sound is spawned
    // Resulting in overlap between plays
    ew.send(
        SfxChannel::play_event(AudioFiles::FireOGG)
            .with_settings(PlaybackSettings::DESPAWN)
            .with_delay_mode(DelayMode::Milliseconds(-500)),
    );
}
