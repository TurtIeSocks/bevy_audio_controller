use bevy::{color::palettes::tailwind, ecs::system::EntityCommands, log::LogPlugin, prelude::*};
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
        .add_systems(
            Update,
            (
                play_sfx,
                volume_buttons::<GlobalChannel>,
                volume_label::<GlobalChannel>,
                volume_buttons::<MusicChannel>,
                volume_label::<MusicChannel>,
                volume_buttons::<SfxChannel>,
                volume_label::<SfxChannel>,
            ),
        )
        .run();
}

const TEXT_SIZE: f32 = 40.;
const LABEL_BACKGROUND: Srgba = tailwind::ORANGE_900;

#[derive(Component)]
struct VolumeUpButton;
#[derive(Component)]
struct VolumeDownButton;
#[derive(Component)]
struct VolumeLabel;

fn volume_buttons<Channel: ACBounds>(
    up_query: Query<&Interaction, (Changed<Interaction>, With<VolumeUpButton>, With<Channel>)>,
    down_query: Query<&Interaction, (Changed<Interaction>, With<VolumeDownButton>, With<Channel>)>,
    mut ew: EventWriter<SettingsEvent<Channel>>,
    settings: Res<ChannelSettings<Channel>>,
) {
    let mut current = settings.get_channel_volume();
    for interaction in up_query.iter() {
        if interaction == &Interaction::Pressed {
            current = (current + 0.05).min(1.0);
        }
    }
    for interaction in down_query.iter() {
        if interaction == &Interaction::Pressed {
            current = (current - 0.05).max(0.0);
        }
    }
    ew.send(SettingsEvent::<Channel>::new().with_volume(current));
}

fn volume_label<Channel: ACBounds>(
    mut text_query: Query<&mut Text, (With<VolumeLabel>, With<Channel>)>,
    settings: Res<ChannelSettings<Channel>>,
) {
    for mut text in &mut text_query {
        text.sections[0].value = format!("{:.0}%", settings.get_channel_volume() * 100.0);
    }
}

fn setup(mut commands: Commands, mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.),
                ..default()
            },
            z_index: ZIndex::Global(25),
            ..default()
        })
        .with_children(|parent| {
            build_row(parent).with_children(|parent| {
                build_header(parent, "Global Volume");
            });
            build_row(parent).with_children(|parent| {
                build_audio_row::<GlobalChannel>(parent);
            });
            build_row(parent).with_children(|parent| {
                build_header(parent, "Music Volume");
            });
            build_row(parent).with_children(|parent| {
                build_audio_row::<MusicChannel>(parent);
            });
            build_row(parent).with_children(|parent| {
                build_header(parent, "SFX Volume");
            });
            build_row(parent).with_children(|parent| {
                build_audio_row::<SfxChannel>(parent);
            });
        });

    // Adjusting Music & Global will affect this sound
    ew.send(
        MusicChannel::play_event(AudioFiles::BackgroundOGG).with_settings(PlaybackSettings::LOOP),
    );
}

fn play_sfx(
    mut sfx_ew: EventWriter<PlayEvent<SfxChannel>>,
    mut global_ew: EventWriter<PlayEvent<GlobalChannel>>,
) {
    // Adjusting SFX & Global will affect this sound
    sfx_ew
        .send(SfxChannel::play_event(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN));

    // Adjusting Global will affect this sound
    global_ew.send(
        GlobalChannel::play_event(AudioFiles::SprayOGG).with_settings(PlaybackSettings::DESPAWN),
    );
}

fn build_header(parent: &mut ChildBuilder, text: &str) {
    parent
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::top(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text.to_string(),
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ));
        });
}

fn build_row<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(5.),
            ..default()
        },
        ..default()
    })
}

fn build_audio_row<Channel: ACBounds>(parent: &mut ChildBuilder) {
    build_button(parent, "<", (VolumeDownButton, Channel::default()));
    build_label(parent, "100", (VolumeLabel, Channel::default()));
    build_button(parent, ">", (VolumeUpButton, Channel::default()));
}

fn build_label(parent: &mut ChildBuilder, text: &str, marker: impl Bundle) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            border_radius: BorderRadius::all(Val::Px(5.)),
            background_color: LABEL_BACKGROUND.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("{}%", text),
                    TextStyle {
                        font_size: TEXT_SIZE,
                        ..default()
                    },
                ),
                marker,
            ));
        });
}

fn build_button(parent: &mut ChildBuilder, text: &str, marker: impl Bundle) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(5.)),
                ..default()
            },
            marker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text.to_string(),
                TextStyle {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ));
        });
}
