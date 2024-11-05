# Bevy Audio Controller

<!--
[![license](https://img.shields.io/crates/l/bevy_audio_controller)](https://github.com/TurtIeSocks/bevy_audio_controller#license) -->

An extremely convenient plugin that provides a solid audio implementation for Bevy with very minimal boilerplate!

## Features

### Event Orientated

- Playing a sound is usually the result of a trigger, spawning audio via an event feels natural!
- Avoids unnecessary spawns/inserts of audio components, increasing performance
- Still includes support for ECS design patterns

### Automatic Audio File Detection at Build Time

- The build script traverses through your Bevy assets folder and builds convenient structs, enums, component markers, and traits based on the audio files that are compatible with the specified Cargo features
- Removes the need to ever use the `AssetServer` directly and provides a convenient enum so you can avoid "magic strings" in your code

### Channels

- Provides `register_audio_channel` trait to allow you to easily add multiple audio channels to your app
- Each channel gets its own settings, events, and can be controlled independently with convenient APIs
- `AudioChannel` derive macro adds convenient methods to the channel marker struct

### Tracks

- Defaults for individual tracks can be set per channel
- Settings can still be overridden on a per event basis

## Usage

```rust
use bevy::{prelude::*, audio::PlaybackSettings};
use bevy_audio_controller::prelude::*;

#[derive(Component, Default, AudioChannel)]
struct SfxChannel;

type SfxEvent = AudioEvent<SfxChannel>;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to the app, since it is generic, you can add as many channels as you want
        .add_plugins(AudioControllerPlugin)
        .register_audio_channel::<SfxChannel>()
        .add_systems(Update, play_fire)
        .run();
}

fn play_fire(mut ew: EventWriter<SfxEvent>) {
    // even though this is called on every frame, it will only be played once the previous clip has finished
    ew.send(SfxEvent::new(AudioFiles::FireOGG).with_settings(PlaybackSettings::DESPAWN));
}
```

## Cargo Features

### `default`

None

### `inspect`

Adds additional reflection traits to the structs used by this plugin to make them available in `bevy-egui-inspector`

**Requires that channel components must also derive `Reflect`**

```rust
// If you are using the `inspect` feature conditionally, you can use the following pattern
#[derive(Component, Default, AudioChannel)]
#[cfg_attr(feature = "inspect", derive(Reflect))]
#[cfg_attr(feature = "inspect", reflect(Component))]
struct SfxChannel;

// Otherwise, this is fine
#[derive(Component, Default, AudioChannel, Reflect)]
#[reflect(Component)]
struct MusicChannel;
```

### `mp3`

Enables support for MP3 audio files.

### `ogg`

Enables support for OGG audio files

### `flac`

Enables support for FLAC audio files

### `wav`

Enables support for WAV audio files

### `all-codecs`

Enables support for all audio codecs

## Examples

All examples require `--features="ogg"` flag to work. If you would like to view more details with bevy-egui-inspector, run with `--all-features` instead.

### Basic

Demonstrates:

- Utilizing the global audio channel
- Playing an audio clip using the plugin

Inputs:

- Space Bar: Toggles between using the plugin and the standard Bevy audio spawn

```sh
  cargo run --example basic --features="ogg"
```

### Channels

Demonstrates:

- Spawning multiple audio channels
- Playing an audio clip using the plugin

```sh
  cargo run --example channels --features="ogg"
```

### Event Options

Demonstrates:

- Set the volume for a channel
- Set the default PlaybackSettings for a channel
- Set individual PlaybackSettings for a track
- Insert a track into an entity
- Add a track as a child to another entity
- Override cache with an immediate play event

Inputs:

- Space Bar: Sends an event to ignore the cache and immediate play a track

```sh
  cargo run --example event_options --features="ogg"
```

### ECS

Demonstrates:

- How to use this plugin with a more traditional ECS design pattern

Inputs:

- Space Bar: Toggles how _not_ to use `DelayMode::Immediate`

```sh
  cargo run --example ecs --features="ogg"
```

### Delays

Demonstrates:

- Demonstrates how to use the `Percent` & `Milliseconds` variations of the `DelayMode` enum for finer control over when a track is played

```sh
  cargo run --example delays --features="ogg"
```

### Volume

Demonstrates:

- Includes a full UI for controlling the volumes of individual channels

Inputs:

- Clicking buttons to adjust volumes

```sh
  cargo run --example volume --features="ogg"
```

### Querying

Demonstrates:

- Query for audio components after they've been inserted if you want to use or modify their components in some way
- Use the unique markers that are generated by the build script at compile time for each audio file

```sh
  cargo run --example querying --features="ogg"
```

## Bevy support table

| bevy | bevy_audio_controller |
| ---- | --------------------- |
| 0.14 | 0.2                   |

## Credits

- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets/tree/main) for inspiration with the [build.rs](./build.rs) script
- [Assets used in the examples](https://yourpalrob.itch.io/)
