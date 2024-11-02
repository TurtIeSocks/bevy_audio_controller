use bevy::{input::common_conditions::input_just_pressed, log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

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
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<MusicChannel>()
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                play_sfx,
                force_play.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .add_systems(PostUpdate, (do_something_with_sfx, do_something_with_fire))
        .run();
}

fn setup(mut commands: Commands, mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    ew.send(PlayEvent::<MusicChannel>::new("background.ogg".into()));
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

fn play_sfx(
    parent_query: Query<Entity, With<SfxParent>>,
    mut ew: EventWriter<PlayEvent<SfxChannel>>,
) {
    if parent_query.is_empty() {
        return;
    }
    let parent_entity = parent_query.single();
    ew.send(
        PlayEvent::<SfxChannel>::from(AudioFiles::FireOGG)
            .with_settings(PlaybackSettings::DESPAWN)
            .with_parent(parent_entity),
    );
    ew.send(
        PlayEvent::<SfxChannel>::new("spray.ogg".into()).with_settings(PlaybackSettings::DESPAWN),
    );
}

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

fn do_something_with_fire(
    sfx_query: Query<(Entity, &Name, &AudioSink), (Added<AudioSink>, With<FireOGG>)>,
) {
    for (entity, name, sink) in sfx_query.iter() {
        sink.set_speed(2.5);
        info!(
            "Fire: {} ({}) is playing at speed {}",
            name,
            entity,
            sink.speed()
        );
    }
}

fn force_play(mut ew: EventWriter<PlayEvent<SfxChannel>>) {
    ew.send(
        PlayEvent::<SfxChannel>::new("fire.ogg".into())
            .with_force()
            .with_settings(PlaybackSettings::DESPAWN),
    );
}
