use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

const ASSET_PATH_VAR: &str = "BEVY_ASSET_PATH";
const OUTPUT_FILE_NAME: &str = "audio_controller.rs";

fn main() {
    cargo_emit::rerun_if_env_changed!(ASSET_PATH_VAR);

    // Check if env variable is set for the assets folder
    if let Some(dir) = env::var(ASSET_PATH_VAR)
        .ok()
        .map(|v| Path::new(&v).to_path_buf())
        .and_then(|path| {
            if path.exists() {
                Some(path)
            } else {
                cargo_emit::warning!(
                    "${} points to an unknown folder: {}",
                    ASSET_PATH_VAR,
                    path.to_string_lossy()
                );
                None
            }
        })
        // Otherwise, search for the target folder and look for an assets folder next to it
        .or_else(|| {
            env::var("OUT_DIR")
                .ok()
                .map(|v| Path::new(&v).to_path_buf())
                .and_then(|path| {
                    for ancestor in path.ancestors() {
                        if let Some(last) = ancestor.file_name() {
                            if last == "target" {
                                return ancestor.parent().map(|parent| {
                                    let imported_dir = parent.join("imported_assets");
                                    if imported_dir.exists() {
                                        imported_dir.join("Default")
                                    } else {
                                        parent.join("assets")
                                    }
                                });
                            }
                        }
                    }
                    None
                })
                .and_then(|path| {
                    if path.exists() {
                        Some(path)
                    } else {
                        cargo_emit::warning!(
                            "Could not find asset folder from Cargo build directory"
                        );
                        None
                    }
                })
        })
    {
        cargo_emit::rerun_if_changed!(dir.to_string_lossy());
        // cargo_emit::warning!("Asset folder found: {}", dir.to_string_lossy());

        let out_dir = env::var_os("OUT_DIR").unwrap();

        let mut marker_file = File::create(Path::new(&out_dir).join(OUTPUT_FILE_NAME)).unwrap();
        let mut files = Vec::new();

        let building_for_wasm = std::env::var("CARGO_CFG_TARGET_ARCH") == Ok("wasm32".to_string());

        visit_dirs(&dir)
            .iter()
            .map(|path| (path, path.strip_prefix(&dir).unwrap()))
            .for_each(|(full_path, path)| {
                let mut path = path.to_string_lossy().to_string();
                if building_for_wasm {
                    // building for wasm. replace paths with forward slash in case we're building from windows
                    path = path.replace(std::path::MAIN_SEPARATOR, "/");
                }
                cargo_emit::rerun_if_changed!(full_path.to_string_lossy());
                if let Some(duration) = get_audio_duration(&full_path) {
                    files.push(AudioFile { path, duration });
                }
            });

        // Write the markers
        marker_file
            .write_all(
                format!(
                    r#"pub mod markers {{
    #![allow(unused)]
    #[cfg(feature = "inspect")]
    use bevy::{{ecs::reflect::ReflectComponent, reflect::Reflect}};

{}
}}
"#,
                    files
                        .iter()
                        .map(|f| f.get_marker_struct())
                        .collect::<String>()
                )
                .as_ref(),
            )
            .unwrap();

        // Write the insert_audio_track trait for entity commands so we can do some jank component inserts
        marker_file
            .write_all(
                format!(
                    r#"
mod ac_traits {{
    #![allow(unused)]
    use super::{{markers::*, audio_files::*}};

    pub(crate) trait CommandAudioTracks {{
        fn insert_audio_track(&mut self, id: &AudioFiles) -> &mut Self;
        fn remove_audio_track(&mut self, id: &AudioFiles) -> &mut Self;
    }}

    impl<'a> CommandAudioTracks for bevy::ecs::system::EntityCommands<'a> {{
        fn insert_audio_track(&mut self, id: &AudioFiles) -> &mut bevy::ecs::system::EntityCommands<'a> {{
            match id {{
                {}
                AudioFiles::Unknown => self,
            }}
        }}

        fn remove_audio_track(&mut self, id: &AudioFiles) -> &mut bevy::ecs::system::EntityCommands<'a> {{
            match id {{
                {}
                AudioFiles::Unknown => self,
            }}
        }}
    }}
}}
"#,
                    files
                        .iter()
                        .map(|f| f.insert_audio_track_impl())
                        .collect::<Vec<_>>()
                        .join("\n                "),  files
                        .iter()
                        .map(|f| f.remove_audio_track_impl())
                        .collect::<Vec<_>>()
                        .join("\n                ")
                )
                .as_ref(),
            )
            .unwrap();

        // Write the enum for the audio files
        marker_file
            .write_all(
                format!(
                    r#"
pub mod audio_files {{
    #![allow(unused)]
    
    #[derive(Debug, Default)]
    #[cfg_attr(feature = "inspect", derive(bevy::reflect::Reflect))]
    pub struct AudioFile {{
        pub path: &'static str,
        pub duration: f32,
    }}

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "inspect", derive(bevy::reflect::Reflect))]
    pub enum AudioFiles {{
        #[default]
        Unknown,
        {}
    }}

    impl ToString for AudioFiles {{
        fn to_string(&self) -> String {{
            match self {{
                {}
                Self::Unknown => "Unknown",
            }}
            .to_string()
        }}
    }}

    impl From<&str> for AudioFiles {{
        fn from(file_name: &str) -> Self {{
            match file_name.replace('\\', "/").as_str() {{
                {}
                unknown => {{
                    bevy::log::warn!("Unknown audio file '{{}}' requested", unknown);
                    AudioFiles::Unknown
                }},
            }}
        }}
    }}

    impl From<String> for AudioFiles {{
        fn from(file_name: String) -> Self {{
            Self::from(&file_name)
        }}
    }}

    impl From<&String> for AudioFiles {{
        fn from(file_name: &String) -> Self {{
            Self::from(file_name.as_str())
        }}
    }}

    impl From<AudioFiles> for &str {{
        fn from(file_name: AudioFiles) -> &'static str {{
            match file_name {{
                {}
                unknown => {{
                    bevy::log::warn!("Unknown audio file '{{:?}}' requested", unknown);
                    "Unknown"
                }}
            }}
        }}
    }}

    impl From<&bevy::core::Name> for AudioFiles {{
        fn from(name: &bevy::core::Name) -> Self {{
            Self::from(&name.to_string())
        }}
    }}

    impl AudioFiles {{
        {}

        pub fn get(&self) -> AudioFile {{
            match self {{
                {}
                Self::Unknown => {{
                    bevy::log::warn!("Unknown audio file requested");
                    AudioFile::default()
                }},
            }}
        }}

        pub fn get_duration(&self) -> f32 {{
            match self {{
                {}
                Self::Unknown => {{
                    bevy::log::warn!("Unknown audio duration requested");
                    0.0
                }},
            }}
        }}

        pub fn get_file_name(&self) -> &'static str {{
            match self {{
                {}
                Self::Unknown => {{
                    bevy::log::warn!("Unknown audio file name requested");
                    ""
                }},
            }}
        }}
    }}
}}
"#,
                    files
                        .iter()
                        .map(|f| f.enum_creator())
                        .collect::<Vec<_>>()
                        .join("\n        "),
                    files
                        .iter()
                        .map(|f| f.get_enum_file_match())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.get_enum_match())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.get_enum_file_match())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.audio_file_struct())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.get())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.get_duration())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                    files
                        .iter()
                        .map(|f| f.get_file_name())
                        .collect::<Vec<_>>()
                        .join("\n                "),
                )
                .as_ref(),
            )
            .unwrap();

        marker_file
            .write_all(
                format!(
                    r#"
mod ac_assets {{
    #![allow(unused)]

    use super::audio_files::AudioFiles;
    #[cfg(feature = "inspect")]
    use bevy::{{ecs::reflect::ReflectResource, reflect::Reflect}};

    pub(super) fn load_assets(asset_server: bevy::ecs::system::Res<bevy::asset::AssetServer>, mut internal_loader: bevy::ecs::system::ResMut<AssetLoader>) {{
        {}
    }}

    #[derive(Default, bevy::ecs::system::Resource)]
    #[cfg_attr(feature = "inspect", derive(Reflect))]
    #[cfg_attr(feature = "inspect", reflect(Resource))]
    pub(super) struct AssetLoader {{
        {}
    }}

    impl AssetLoader {{
        pub(super) fn get(&self, id: &AudioFiles) -> Option<bevy::asset::Handle<bevy::audio::AudioSource>> {{
            match id {{
                {}
                AudioFiles::Unknown => None,
            }}
        }}
    }}
}}
"#,files.iter().map(|f| f.asset_loader()).collect::<Vec<_>>().join("\n    "), files.iter().map(|f| f.asset_field()).collect::<Vec<_>>().join("\n    "),files.iter().map(|f| f.asset_getter()).collect::<Vec<_>>().join("\n    ")
                )
                .as_ref(),
            )
            .unwrap();
    } else if std::env::var("DOCS_RS").is_ok() {
        //         let out_dir = env::var_os("OUT_DIR").unwrap();
        //         let dest_path = Path::new(&out_dir).join("audio_lengths.rs");

        //         let mut file = File::create(dest_path).unwrap();
        //         file.write_all(
        //             "/// Generated function that will return the length of the input audio file. It does not panic if a file is missing but will log a warning.
        // fn include_all_assets(registry: impl EmbeddedRegistry){}"
        //                 .as_ref(),
        //         )
        // .unwrap();
    } else {
        cargo_emit::warning!(
            "Could not find asset folder, please specify its path with ${}",
            ASSET_PATH_VAR
        );
        panic!("No asset folder found");
    }
}

struct AudioFile {
    path: String,
    duration: f32,
}

impl AudioFile {
    // fn get_length_match(&self) -> String {
    //     format!(
    //         r#"        {:?} => Some(markers::{}::DURATION),"#,
    //         self.path,
    //         self.pascal_case()
    //     )
    // }

    fn get_enum_match(&self) -> String {
        let struct_name = self.pascal_case();
        format!(
            "path if path == Self::{}.path => AudioFiles::{},",
            self.snake_case().to_uppercase(),
            struct_name
        )
    }

    fn get_enum_file_match(&self) -> String {
        format!(
            "AudioFiles::{} => AudioFiles::{}.path,",
            self.pascal_case(),
            self.snake_case().to_uppercase(),
        )
    }

    fn insert_audio_track_impl(&self) -> String {
        let struct_name = self.pascal_case();
        format!(
            r#"AudioFiles::{} => self.insert({}::default()),"#,
            struct_name, struct_name
        )
    }

    fn remove_audio_track_impl(&self) -> String {
        let struct_name = self.pascal_case();
        format!(
            r#"AudioFiles::{} => self.remove::<{}>(),"#,
            struct_name, struct_name
        )
    }

    fn enum_creator(&self) -> String {
        format!("{},", self.pascal_case())
    }

    fn asset_field(&self) -> String {
        let field_name = self.snake_case();
        format!(
            r#"        pub(super) {}: bevy::asset::Handle<bevy::audio::AudioSource>,"#,
            field_name
        )
    }

    fn asset_loader(&self) -> String {
        format!(
            r#"        internal_loader.{} = asset_server.load(AudioFiles::{}.get_file_name());"#,
            self.snake_case(),
            self.pascal_case()
        )
    }

    fn asset_getter(&self) -> String {
        format!(
            r#"                AudioFiles::{} => Some(self.{}.clone()),"#,
            self.pascal_case(),
            self.snake_case()
        )
    }

    fn get(&self) -> String {
        format!(
            "        Self::{} =>  Self::{},",
            self.pascal_case(),
            self.snake_case().to_uppercase(),
        )
    }

    fn get_duration(&self) -> String {
        format!(
            "        Self::{} =>  Self::{}.duration,",
            self.pascal_case(),
            self.snake_case().to_uppercase(),
        )
    }

    fn get_file_name(&self) -> String {
        format!(
            "        Self::{} =>  Self::{}.path,",
            self.pascal_case(),
            self.snake_case().to_uppercase(),
        )
    }

    fn audio_file_struct(&self) -> String {
        format!(
            "    pub const {}: AudioFile = AudioFile {{ path: {:?}, duration: {} }};",
            self.snake_case().to_uppercase(),
            self.path,
            self.duration
        )
    }

    fn get_marker_struct(&self) -> String {
        let struct_name = self.pascal_case();
        format!(
            r#"
    /// Marker for the audio file: {:?}
    /// 
    /// This is not meant to be inserted or spawned directly outside of the plugin internals
    /// 
    /// Only use them for querying
    #[derive(Debug, bevy::ecs::component::Component, Default)]
    #[cfg_attr(feature = "inspect", derive(Reflect))]
    #[cfg_attr(feature = "inspect", reflect(Component))]
    pub struct {};
"#,
            self.path, struct_name,
        )
    }

    fn pascal_case(&self) -> String {
        let mut parts: Vec<String> = self
            .path
            .split(|c: char| c.is_whitespace() || "-_.\\".contains(c))
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            })
            .collect();

        if let Some(extension) = parts.last_mut() {
            *extension = extension.to_uppercase();
        }

        parts.concat()
    }

    fn snake_case(&self) -> String {
        let mut snake_case = String::new();
        let name = self.path.replace(&['/', '\\', '.', '-'][..], "_");
        let mut prev_char = '\0';
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 && prev_char != '_' {
                snake_case.push('_');
            }
            snake_case.push(ch.to_ascii_lowercase());
            prev_char = ch;
        }
        snake_case
    }
}

fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut collected = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collected.append(&mut visit_dirs(&path));
            } else {
                if is_supported_audio_file(&path) {
                    collected.push(path);
                }
            }
        }
    }
    collected
}

fn is_supported_audio_file(path: &Path) -> bool {
    #[allow(unused_mut)]
    let mut formats = Vec::<&str>::new();
    #[cfg(feature = "flac")]
    formats.push("flac");
    #[cfg(feature = "mp3")]
    formats.push("mp3");
    #[cfg(feature = "ogg")]
    formats.push("ogg");
    #[cfg(feature = "wav")]
    formats.push("wav");
    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        formats.contains(&ext)
    } else {
        return false;
    }
}

fn get_audio_duration(path: &Path) -> Option<f32> {
    let file = std::fs::File::open(path).ok()?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();

    let probed = get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .ok()?;
    let mut format = probed.format;

    if let Some(track) = format.default_track() {
        let codec_params = &track.codec_params;
        let sample_rate = codec_params.sample_rate? as f32;

        let mut decoder = symphonia::default::get_codecs()
            .make(&codec_params, &Default::default())
            .ok()?;
        let mut total_frames = 0;

        while let Ok(packet) = format.next_packet() {
            if let Ok(decoded) = decoder.decode(&packet) {
                total_frames += decoded.frames();
            }
        }

        Some(total_frames as f32 / sample_rate)
    } else {
        None
    }
}
