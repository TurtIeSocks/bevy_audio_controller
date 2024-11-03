use bevy::{input::common_conditions::input_just_pressed, log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

#[derive(Component, Default)]
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
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Press SPACE to force a sound effect to override the cache",
                    TextStyle {
                        font_size: 40.0,
                        ..Default::default()
                    },
                )
                .with_justify(JustifyText::Center),
                ..Default::default()
            });
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
        PlayEvent::<SfxChannel>::from(AudioFiles::FireOGG)
            .with_settings(PlaybackSettings::DESPAWN)
            .with_parent(parent_entity),
    );
    ew.send(
        PlayEvent::<SfxChannel>::new("spray.ogg".into())
            .with_settings(PlaybackSettings::REMOVE)
            .with_entity(player_entity),
    );
}

fn force_play(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    ew.send(
        PlayEvent::<SfxChannel>::new(AudioFiles::FireOGG)
            .with_force()
            .with_settings(PlaybackSettings::DESPAWN),
    );
}
