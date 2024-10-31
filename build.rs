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
        let dest_path = Path::new(&out_dir).join("audio_lengths.rs");

        let mut file = File::create(dest_path).unwrap();

        file.write_all(
            "/// Function for getting the length of an audio file by its path.
fn get_audio_file_length(file_name: &str) -> Option<f32> {\n    match file_name {\n"
                .as_ref(),
        )
        .unwrap();

        let building_for_wasm = std::env::var("CARGO_CFG_TARGET_ARCH") == Ok("wasm32".to_string());

        visit_dirs(&dir)
            .iter()
            .map(|path| (path, path.strip_prefix(&dir).unwrap()))
            .for_each(|(fullpath, path)| {
                let mut path = path.to_string_lossy().to_string();
                if building_for_wasm {
                    // building for wasm. replace paths with forward slash in case we're building from windows
                    path = path.replace(std::path::MAIN_SEPARATOR, "/");
                }
                cargo_emit::rerun_if_changed!(fullpath.to_string_lossy());

                if let Some(duration) = get_audio_duration(&fullpath) {
                    file.write_all(
                        format!(
                            r#"        {:?} => Some({}),
"#,
                            path, duration
                        )
                        .as_ref(),
                    )
                    .unwrap();
                } else {
                    cargo_emit::warning!(
                        "Could not get duration for audio file: {}",
                        fullpath.to_string_lossy()
                    );
                }
            });

        file.write_all(format!("        not_found => {{\n            bevy::utils::tracing::warn!(\"Audio length not found for {{not_found}}\");\n            None\n        }}\n    }}\n}}\n").as_ref())
            .unwrap();
    } else if std::env::var("DOCS_RS").is_ok() {
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("audio_lengths.rs");

        let mut file = File::create(dest_path).unwrap();
        file.write_all(
            "/// Generated function that will return the length of the input audio file. It does not panic if a file is missing but will log a warning.
fn include_all_assets(registry: impl EmbeddedRegistry){}"
                .as_ref(),
        )
        .unwrap();
    } else {
        cargo_emit::warning!(
            "Could not find asset folder, please specify its path with ${}",
            ASSET_PATH_VAR
        );
        panic!("No asset folder found");
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
