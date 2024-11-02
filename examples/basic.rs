use bevy::{
    input::common_conditions::{input_just_pressed, input_toggle_active},
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

#[derive(Component, Default)]
struct SfxChannel;

/// Type alias for the SFX audio event to minimize boilerplate
type SfxEvent = PlayEvent<SfxChannel>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AudioControllerPlugin::<SfxChannel>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                play_with_plugin.run_if(input_toggle_active(true, KeyCode::Space)),
                play_without_plugin.run_if(input_toggle_active(false, KeyCode::Space)),
                despawn_on_change.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
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
                    "Press SPACE to toggle between\nplugin and non-plugin audio",
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

fn play_with_plugin(mut ew: EventWriter<SfxEvent>) {
    ew.send(SfxEvent::new("fire.ogg"));
}

fn play_without_plugin(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Despawn,
            ..Default::default()
        },
        source: asset_server.load("fire.ogg"),
        ..Default::default()
    });
}

/// Only relevant to this example to clean up the audio from the previous state
fn despawn_on_change(mut commands: Commands, query: Query<Entity, With<AudioSink>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
