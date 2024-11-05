use std::marker::PhantomData;

use bevy::{
    audio::PlaybackSettings,
    ecs::{entity::Entity, event::Event},
};

use crate::{audio_files::AudioFiles, bounds::ACBounds, delay_mode::DelayMode};

/// An event for playing an audio file on a channel
///
/// # Example
/// ```
/// use bevy_audio_controller::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(AudioControllerPlugin)
///         .add_systems(Update, play)
///         .run();
/// }
///
/// fn play(mut play_ew: EventWriter<GlobalPlayEvent>) {
///     let event = GlobalPlayEvent::new(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN);
///     play_ew.send(event);
/// }
/// ```

#[derive(Event)]
pub struct PlayEvent<T: ACBounds> {
    pub(super) id: AudioFiles,
    pub(super) entity: Option<Entity>,
    pub(super) child: bool,
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) delay_mode: Option<DelayMode>,
    _marker: PhantomData<T>,
}

impl<T: ACBounds> PlayEvent<T> {
    /// Create a new PlayEvent with the given audio file
    pub fn new(id: AudioFiles) -> Self {
        Self {
            id,
            entity: None,
            settings: None,
            delay_mode: None,
            child: false,
            _marker: PhantomData::<T>,
        }
    }

    /// Set the entity to play the audio on
    pub fn with_entity(self, entity: Entity) -> Self {
        Self {
            entity: Some(entity),
            ..self
        }
    }

    /// Specify the [PlaybackSettings] for the track, overrides the default and channel settings
    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    /// Set the delay mode for the track, overrides the default and channel settings
    pub fn with_delay_mode(mut self, delay_mode: DelayMode) -> Self {
        self.delay_mode = Some(delay_mode);
        self
    }

    /// Set the audio to play as a child of the entity
    ///
    /// `with_entity` must be called before this otherwise it will panic
    pub fn as_child(mut self) -> Self {
        self.entity.expect(
            "Cannot set as child without an entity, try calling `with_entity(entity)` first",
        );
        self.child = true;
        self
    }
}

impl<Channel: ACBounds> From<AudioFiles> for PlayEvent<Channel> {
    fn from(id: AudioFiles) -> Self {
        Self::new(id)
    }
}

/// An event for changing the settings of a channel
///
/// Including track specific settings
///
/// # Example
/// ```
/// use bevy_audio_controller::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugins(AudioControllerPlugin)
///         .add_systems(Startup, setup)
///         .run();
/// }
///
/// fn setup(mut ew: EventWriter<SettingsEvent<SfxChannel>>) {
///     // Set the volume for the channel
///     let vol_event = SfxChannel::settings_event().with_volume(0.5);
///
///     // Set the default playback settings for the channel
///     let default_settings_event =
///         SfxChannel::settings_event().with_settings(PlaybackSettings::DESPAWN);
///
///     // Sets (and overwrites!) the playback settings for all tracks in your assets folder for the channel
///     let all_track_settings_event = SfxChannel::settings_event()
///         .with_settings(PlaybackSettings::REMOVE)
///         .all();
///
///     // Set the playback settings for a specific track in the channel
///     let track_settings_event = SfxChannel::settings_event()
///         .with_settings(PlaybackSettings::LOOP)
///         .with_track(AudioFiles::BackgroundOGG);
///
///     ew.send_batch(vec![
///         vol_event,
///         default_settings_event,
///         all_track_settings_event,
///         track_settings_event,
///     ]);
/// }
/// ```
#[derive(Event)]
pub struct SettingsEvent<Channel: ACBounds> {
    pub(super) settings: Option<PlaybackSettings>,
    pub(super) volume: Option<f32>,
    pub(super) track: Option<AudioFiles>,
    pub(super) delay_mode: Option<DelayMode>,
    pub(super) all: bool,
    _marker: PhantomData<Channel>,
}

impl<Channel: ACBounds> SettingsEvent<Channel> {
    pub fn new() -> Self {
        Self {
            track: None,
            settings: None,
            volume: None,
            delay_mode: None,
            all: false,
            _marker: PhantomData::<Channel>,
        }
    }

    /// Sets the volume for the channel
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = Some(volume);
        self
    }

    /// When called on its own without `with_track` or `all`, this sets the default [PlaybackSettings] for the channel
    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = Some(settings);
        self
    }

    /// When called on its own without `with_track` or `all`, this sets the default [DelayMode] for the channel
    pub fn with_delay_mode(mut self, delay_mode: DelayMode) -> Self {
        self.delay_mode = Some(delay_mode);
        self
    }

    /// Instead applies the specified setting or delay_mode to a specific track
    pub fn with_track(mut self, id: AudioFiles) -> Self {
        if self.all {
            panic!("Do set all and a specific track at the same time, either call `all()` or `with_track()`");
        }
        self.track = Some(id);
        self
    }

    /// Instead applies the specified setting or delay_mode to every track in the channel
    pub fn all(mut self) -> Self {
        if self.track.is_some() {
            panic!("Do set all and a specific track at the same time, either call `all()` or `with_track()`");
        }
        self.all = true;
        self
    }
}
