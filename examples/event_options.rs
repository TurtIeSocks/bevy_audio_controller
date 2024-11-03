use bevy::{input::common_conditions::input_just_pressed, log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

mod helpers;

#[derive(Component, Default, AudioChannel)]
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
        .add_systems(Startup, setup)
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
            ));
        });
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
        SfxChannel::new_play_event(AudioFiles::FireOGG)
            .with_settings(PlaybackSettings::DESPAWN)
            .with_entity(parent_entity)
            .as_child(),
    );
    ew.send(
        SfxChannel::new_play_event("spray.ogg".into())
            .with_settings(PlaybackSettings::REMOVE)
            .with_entity(player_entity),
    );
}

fn force_play(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    ew.send(
        SfxChannel::new_play_event(AudioFiles::FireOGG)
            .with_delay_mode(DelayMode::Immediate)
            .with_settings(PlaybackSettings::DESPAWN),
    );
}
