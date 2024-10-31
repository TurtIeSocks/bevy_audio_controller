use bevy::{
    input::common_conditions::{input_just_pressed, input_toggle_active},
    log::LogPlugin,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::{AudioControllerEvent, AudioControllerPlugin};

#[derive(Component, Default, Reflect)]
struct MusicChannel;

#[derive(Component, Default, Reflect)]
struct SfxChannel;

#[derive(Component, Default, Reflect)]
struct SfxParent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "symphonia_core=warn,wgpu=error,symphonia_bundle_mp3=warn".to_string(),
            ..Default::default()
        }))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            AudioControllerPlugin::<MusicChannel>::new(
                0.5,
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    ..Default::default()
                },
            ),
            AudioControllerPlugin::<SfxChannel>::default()
                .with_fallback_delay_time(0.25)
                .with_settings(PlaybackSettings::REMOVE),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                play_fire_sound,
                force_play.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .add_systems(PostUpdate, do_something_with_sfx)
        .run();
}

fn setup(mut commands: Commands, mut ew: EventWriter<AudioControllerEvent<MusicChannel>>) {
    ew.send(AudioControllerEvent::<MusicChannel>::new("background.ogg"));
    commands.spawn((Name::new("SFX Container"), SfxParent));
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

fn play_fire_sound(
    parent_query: Query<Entity, With<SfxParent>>,
    mut ew: EventWriter<AudioControllerEvent<SfxChannel>>,
) {
    if parent_query.is_empty() {
        return;
    }
    let parent_entity = parent_query.single();
    ew.send(AudioControllerEvent::<SfxChannel>::new("fire.ogg").with_parent(parent_entity));
}

fn do_something_with_sfx(
    sfx_query: Query<(Entity, &Name, &AudioSink), (Added<AudioSink>, With<SfxChannel>)>,
) {
    for (entity, name, sink) in sfx_query.iter() {
        sink.set_volume(0.5);
        info!(
            "Sfx: {} ({}) is playing at volume {}",
            name,
            entity,
            sink.volume()
        );
    }
}

fn force_play(mut ew: EventWriter<AudioControllerEvent<SfxChannel>>) {
    ew.send(AudioControllerEvent::<SfxChannel>::new("fire.ogg").with_force());
}
