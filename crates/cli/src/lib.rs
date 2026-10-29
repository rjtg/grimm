//! Host-side codecs and filesystem access. The engine has no format/I/O dependency.
use grimm_engine::Engine;
use grimm_semantic::{save::SaveGame, story::Story};
use std::{fmt, fs, path::Path};

#[derive(Debug)]
pub struct FileError(String);
impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for FileError {}

/// Parse and validate before returning a runnable engine. Keep source context in errors.
pub fn parse_story(source: &str, yaml: &str) -> Result<(String, Engine), FileError> {
    let story: Story = serde_yaml_ng::from_str(yaml)
        .map_err(|error| FileError(format!("{source}: invalid YAML story: {error}")))?;
    let title = story.title.clone();
    let engine = Engine::new(story)
        .map_err(|error| FileError(format!("{source}: invalid story: {error}")))?;
    Ok((title, engine))
}

pub fn load_story(path: &Path) -> Result<(String, Engine), FileError> {
    let text = fs::read_to_string(path)
        .map_err(|error| FileError(format!("{}: cannot read story: {error}", path.display())))?;
    parse_story(&path.display().to_string(), &text)
}

pub fn write_save(path: &Path, engine: &Engine) -> Result<(), FileError> {
    let json = serde_json::to_string_pretty(&engine.save())
        .map_err(|error| FileError(format!("cannot encode save: {error}")))?;
    fs::write(path, json)
        .map_err(|error| FileError(format!("{}: cannot write save: {error}", path.display())))
}

pub fn read_save(path: &Path) -> Result<SaveGame, FileError> {
    let json = fs::read_to_string(path)
        .map_err(|error| FileError(format!("{}: cannot read save: {error}", path.display())))?;
    serde_json::from_str(&json)
        .map_err(|error| FileError(format!("{}: invalid JSON save: {error}", path.display())))
}
