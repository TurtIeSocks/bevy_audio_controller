use bevy::{input::common_conditions::input_just_pressed, log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

mod helpers;

#[derive(Component, Default, AudioChannel)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
struct SfxChannel;

#[derive(Component)]
struct SfxParent;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, (setup, set_channel_settings))
        .add_systems(
            Update,
            (
                play_sfx,
                force_play.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Name::new("SFX Container"), SfxParent));
    commands.spawn((Name::new("Player"), Player));
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(helpers::get_container())
        .with_children(|parent| {
            parent.spawn(helpers::get_text(
                "Press SPACE to force a sound effect to override the cache",
                40.0,
            ));
        });
}

fn set_channel_settings(mut ew: EventWriter<SettingsEvent<SfxChannel>>) {
    // Set the volume for the channel
    let vol_event = SfxChannel::settings_event().with_volume(0.5);

    // Set the default playback settings for the channel
    let default_settings_event =
        SfxChannel::settings_event().with_settings(PlaybackSettings::DESPAWN);

    // Sets (and overwrites!) the playback settings for all tracks in your assets folder for the channel
    let all_track_settings_event = SfxChannel::settings_event()
        .with_settings(PlaybackSettings::REMOVE)
        .all();

    // Set the playback settings for a specific track in the channel
    let track_settings_event = SfxChannel::settings_event()
        .with_settings(PlaybackSettings::LOOP)
        .with_track(AudioFiles::BackgroundOGG);

    ew.send_batch(vec![
        vol_event,
        default_settings_event,
        all_track_settings_event,
        track_settings_event,
    ]);
}

fn play_sfx(
    mut ew: EventWriter<PlayEvent<SfxChannel>>,
    parent_query: Query<Entity, With<SfxParent>>,
    player_query: Query<Entity, With<Player>>,
) {
    if parent_query.is_empty() || player_query.is_empty() {
        return;
    }
    let parent_entity = parent_query.single();
    let player_entity = player_query.single();
    ew.send(
        SfxChannel::play_event(AudioFiles::FireOGG)
            // Overrides the default settings for this track
            .with_settings(PlaybackSettings::REMOVE)
            .with_entity(parent_entity)
            .with_delay_mode(DelayMode::Wait)
            .as_child(),
    );
    ew.send(
        SfxChannel::play_event("spray.ogg".into())
            .with_settings(PlaybackSettings::REMOVE)
            .with_entity(player_entity),
    );
}

fn force_play(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    ew.send(
        SfxChannel::play_event(AudioFiles::FireOGG)
            .with_delay_mode(DelayMode::Immediate)
            .with_settings(PlaybackSettings::DESPAWN),
    );
}
