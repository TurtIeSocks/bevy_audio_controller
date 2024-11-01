# Bevy Audio Controller

<!--
[![license](https://img.shields.io/crates/l/bevy_audio_controller)](https://github.com/TurtIeSocks/bevy_audio_controller#license) -->

This plugin can help reduce required boilerplate code and increase performance, especially when targeting WASM, by providing a controller that determines whether or not an audio clip should play. I originally used this in my [Spooky Game Jam Entry](https://turtiesocks.github.io/pumpkin-palooza/) to only play a single attack or hit sound, even if several hundred enemies all attacked at once.

The build script will automatically filter through your assets folder and determine the lengths of all of your audio assets for use within the plugin. This allows the plugin to make sure it only plays an audio clip once the current entity of the same file has finished playing.

Be sure to check out the examples to see how to use this plugin.

## Usage

```rust
#[derive(Component, Default)]
struct SfxChannel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to the app, since it is generic, you can add as many channels as you want
        .add_plugins(AudioControllerPlugin::<SfxChannel>::default())
        .add_systems(Update, play_with_plugin)
        .run();
}

fn play_with_plugin(mut ew: EventWriter<SfxEvent>) {
    // even though this is called on every frame, it will only be played once the previous clip has finished
    ew.send(SfxEvent::new("fire.ogg"));
}
```

## Examples

### Basic

Demonstrates:

- Spawn a single audio channel
- Playing an audio clip using the plugin

Inputs:

- Space Bar: Toggles between using the plugin and the standard Bevy audio spawn

```sh
  cargo run --example basic --features="ogg"
```

### Advanced

Demonstrates:

- Spawning multiple audio channels with different settings
- Playing an audio clip using the plugin
- Spawning the audio bundles with a specified parent entity
- Further tweaking the `AudioSink` components after the plugin has spawned them
- How the `inspect` feature can be used to show more information in bevy-egui-inspector

Inputs:

- Space Bar: Force a sound to play and ignore the controller

```sh
  cargo run --example advanced --all-features
```

## Cargo Features

### `default`

None

### `inspect`

Adds additional reflection traits to the structs used by this plugin to make them available in `bevy-egui-inspector`

**Requires that channel components must also derive `Reflect`**

### `mp3`

Enables support for MP3 audio files.

### `ogg`

Enables support for OGG audio files

### `flac`

Enables support for FLAC audio files

### `wav`

Enables support for WAV audio files

## Bevy support table

| bevy | bevy_audio_controller |
| ---- | --------------------- |
| 0.14 | 0.1                   |

## Credits

- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets/tree/main) for inspiration with the [build.rs](./build.rs) script
- [Assets used in the examples](https://yourpalrob.itch.io/)
