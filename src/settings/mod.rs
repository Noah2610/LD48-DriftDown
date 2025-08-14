pub mod prelude {
    pub use super::audio_settings::AudioSettings;
    pub use super::camera_settings::CameraSettings;
    pub use super::lanes_settings::LanesSettings;
    pub use super::objects_settings::{ObjectSettings, ObjectsSettings};
    pub use super::player_settings::PlayerSettings;
    pub use super::savefile_settings::SavefileSettings;
    pub use super::streak_settings::StreakSettings;
    pub use super::zones_settings::{ZoneId, ZoneSettings, ZonesSettings};
    pub use super::Settings;
}

pub mod audio_settings;
pub mod camera_settings;
pub mod entity_components;
pub mod hitbox_config;
pub mod lanes_settings;
pub mod objects_settings;
pub mod player_settings;
pub mod savefile_settings;
pub mod streak_settings;
pub mod zones_settings;

use crate::resource;
use deathframe::amethyst;
use deathframe::components::prelude::Merge;
use prelude::*;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

pub struct Settings {
    pub camera: CameraSettings,
    pub player: PlayerSettings,
    pub objects: ObjectsSettings,
    pub lanes: LanesSettings,
    pub zones: ZonesSettings,
    pub audio: AudioSettings,
    pub savefile: SavefileSettings,
    pub streak: StreakSettings,
}

impl Settings {
    pub fn load() -> deathframe::amethyst::Result<Self> {
        Ok(Self {
            lanes: load_settings("settings/lanes.ron")?,
            camera: load_settings("settings/camera.ron")?,
            player: load_settings("settings/player.ron")?,
            objects: load_settings_dir("settings/objects")?,
            zones: load_settings_dir("settings/zones")?,
            audio: load_settings("settings/audio.ron")?,
            savefile: load_settings("settings/savefile.ron")?,
            streak: load_settings("settings/streak.ron")?,
        })
    }
}

fn load_settings<S, P>(path: P) -> amethyst::Result<S>
where
    for<'de> S: serde::Deserialize<'de>,
    P: fmt::Display,
{
    let file = File::open(resource(path.to_string()))?;
    Ok(ron::de::from_reader(file).map_err(|e| {
        amethyst::Error::from_string(format!(
            "Failed parsing RON settings file: {}\n{:#?}",
            path, e
        ))
    })?)
}

// Functions below copied from deathfloor project.

fn load_settings_dir<T, S>(dirname: S) -> amethyst::Result<T>
where
    for<'de> T: serde::Deserialize<'de> + Merge + Default,
    S: std::fmt::Display,
{
    let path = resource(dirname.to_string());
    let errmsg = format!("No settings files found in {:?}", &path);
    let all_settings = load_configs_recursively_from(path)?;
    let merged_settings = merge_settings(all_settings).unwrap_or_else(|| {
        eprintln!(
            "[WARNING]\n    {}\n    Using default (probably empty settings)",
            errmsg
        );
        T::default()
    });
    Ok(merged_settings)
}

fn load_configs_recursively_from<T>(path: PathBuf) -> amethyst::Result<Vec<T>>
where
    for<'de> T: serde::Deserialize<'de> + Merge,
{
    let mut settings = Vec::new();

    for entry in path.read_dir()? {
        let entry_path = entry?.path();
        if entry_path.is_file() {
            if let Some("ron") = entry_path.extension().and_then(|e| e.to_str())
            {
                let file = File::open(&entry_path)?;
                settings.push(ron::de::from_reader(file).map_err(|e| {
                    amethyst::Error::from_string(format!(
                        "Failed parsing RON settings file: {:?}\n{:#?}",
                        entry_path, e
                    ))
                })?);
            }
        } else if entry_path.is_dir() {
            settings.append(&mut load_configs_recursively_from(entry_path)?);
        }
    }

    Ok(settings)
}

/// Merge `Vec` of settings `T` together.
/// Returns `None` if given `Vec` is empty.
fn merge_settings<T>(all_settings: Vec<T>) -> Option<T>
where
    T: Merge,
{
    let mut merged_settings: Option<T> = None;
    for settings in all_settings {
        if let Some(merged) = merged_settings.as_mut() {
            merged.merge(settings);
        } else {
            merged_settings = Some(settings);
        }
    }
    merged_settings
}
