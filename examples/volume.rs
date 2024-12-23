use bevy::{color::palettes::tailwind, ecs::system::EntityCommands, log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_audio_controller::prelude::*;

#[derive(Component, Default, AudioChannel)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
struct MusicChannel;

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
        .register_audio_channel::<MusicChannel>()
        .register_audio_channel::<SfxChannel>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                play_sfx,
                volume_buttons::<GlobalChannel>,
                volume_label::<GlobalChannel>
                    .run_if(resource_changed::<ChannelSettings<GlobalChannel>>),
                volume_buttons::<MusicChannel>,
                volume_label::<MusicChannel>
                    .run_if(resource_changed::<ChannelSettings<MusicChannel>>),
                volume_buttons::<SfxChannel>,
                volume_label::<SfxChannel>.run_if(resource_changed::<ChannelSettings<SfxChannel>>),
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
        **text = format!("{:.0}%", settings.get_channel_volume() * 100.0);
    }
}

fn setup(mut commands: Commands, mut ew: EventWriter<PlayEvent<MusicChannel>>) {
    commands.spawn(Camera2d::default());
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.),
                ..default()
            },
            GlobalZIndex(25),
        ))
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
        MusicChannel::play_event(AudioFiles::MusicBackgroundOGG)
            .with_settings(PlaybackSettings::LOOP),
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
        .spawn(Node {
            padding: UiRect::top(Val::Px(20.0)),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                Text::new(text),
                TextFont {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ));
        });
}

fn build_row<'a>(parent: &'a mut ChildBuilder) -> EntityCommands<'a> {
    parent.spawn(Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        column_gap: Val::Px(5.),
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
        .spawn((
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(LABEL_BACKGROUND.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(format!("{}%", text)),
                TextFont {
                    font_size: TEXT_SIZE,
                    ..default()
                },
                marker,
            ));
        });
}

fn build_button(parent: &mut ChildBuilder, text: &str, marker: impl Bundle) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(5.)),
            marker,
        ))
        .with_children(|parent: &mut ChildBuilder<'_>| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: TEXT_SIZE,
                    ..default()
                },
            ));
        });
}
