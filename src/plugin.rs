use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    audio::{AudioBundle, PlaybackSettings},
    core::Name,
    ecs::{
        event::EventReader,
        system::{Commands, Res, ResMut},
    },
    hierarchy::BuildChildren,
    time::Time,
};

use crate::{bounds::Bounds, cache::AudioCache, event::AudioControllerEvent};

pub struct AudioControllerPlugin<T: Bounds> {
    settings: PlaybackSettings,
    fallback_delay_time: f32,
    _marker: PhantomData<T>,
}

impl<T: Bounds> Plugin for AudioControllerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioCache::<T>::new(self.fallback_delay_time))
            .add_event::<AudioControllerEvent<T>>()
            .add_systems(Update, self.build_event_reader());

        #[cfg(feature = "inspect")]
        app.register_type::<AudioCache<T>>();
    }
}

impl<T: Bounds> Default for AudioControllerPlugin<T> {
    fn default() -> Self {
        Self {
            settings: PlaybackSettings::DESPAWN,
            fallback_delay_time: 0.25,
            _marker: PhantomData::<T>,
        }
    }
}

impl<T: Bounds> AudioControllerPlugin<T> {
    pub fn new(fallback_delay_time: f32, settings: PlaybackSettings) -> Self {
        Self {
            fallback_delay_time,
            settings,
            _marker: PhantomData::<T>,
        }
    }

    pub fn with_settings(mut self, settings: PlaybackSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn with_fallback_delay_time(mut self, fallback_delay_time: f32) -> Self {
        self.fallback_delay_time = fallback_delay_time;
        self
    }

    fn build_event_reader(
        &self,
    ) -> impl FnMut(
        Commands,
        ResMut<AudioCache<T>>,
        Res<AssetServer>,
        Res<Time>,
        EventReader<AudioControllerEvent<T>>,
    ) {
        let settings = self.settings.clone();
        move |mut commands: Commands,
              mut audio_cache: ResMut<AudioCache<T>>,
              asset_server: Res<AssetServer>,
              time: Res<Time>,
              mut events: EventReader<AudioControllerEvent<T>>| {
            audio_cache.map.iter_mut().for_each(|(_, timer)| {
                timer.tick(time.delta());
            });

            for event in events.read() {
                let id = event.id.to_string();
                let ready = audio_cache.can_play_sfx(&id);

                if ready {
                    audio_cache.reset_entry(id.clone());
                }
                if event.force || ready {
                    let child = commands
                        .spawn((
                            AudioBundle {
                                settings,
                                source: asset_server.load(&id),
                            },
                            T::default(),
                            Name::new(id),
                        ))
                        .id();
                    if let Some(entity) = event.entity {
                        commands.entity(entity).add_child(child);
                    }
                }
            }
        }
    }
}
