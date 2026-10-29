use grimm_adapters::{InputAdapter, OutputAdapter, TextAdapter};
use grimm_cli::{load_story, read_save, write_save};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::Engine;
use grimm_presentation::PresentationCommand;
use std::{env, io, path::Path, process::ExitCode};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    let (title, mut engine) = match args.as_slice() {
        [] => (None, Engine::default()),
        [help] if help == "--help" || help == "-h" => {
            println!(
                "Usage: grimm-cli [play STORY.yaml [--load SAVE.json]]\nSession commands: :save PATH, :load PATH, :quit"
            );
            return Ok(());
        }
        [play, story] if play == "play" => {
            let (title, engine) = load_story(Path::new(story))?;
            (Some(title), engine)
        }
        [play, story, load, save] if play == "play" && load == "--load" => {
            let (title, mut engine) = load_story(Path::new(story))?;
            engine
                .restore(read_save(Path::new(save))?)
                .map_err(|error| {
                    format!("{}: cannot restore: {error}", Path::new(save).display())
                })?;
            (Some(title), engine)
        }
        _ => return Err("Usage: grimm-cli [play STORY.yaml [--load SAVE.json]]".into()),
    };
    let mut adapter = TextAdapter::default();
    let mut renderer = CliRenderer::new(io::stdin().lock(), io::stdout().lock());
    if let Some(title) = title {
        renderer.render(&[
            PresentationCommand::Text(title),
            PresentationCommand::Text("Commands: :save PATH, :load PATH, :quit".into()),
        ])?;
    }
    loop {
        let output = engine.output();
        renderer.render(&adapter.present(&output))?;
        if output.ending.is_some() {
            return Ok(());
        }
        if output.offers.is_empty() {
            return Err("Story has no executable choices in the current state.".into());
        }
        let Some(input) = renderer.read_input()? else {
            return Ok(());
        };
        let line = input.line.trim();
        if line == ":quit" {
            return Ok(());
        }
        if line.starts_with(':') {
            let (command, path) = line
                .split_once(char::is_whitespace)
                .map_or((line, ""), |(command, path)| (command, path.trim()));
            let message = match (command, path) {
                (":save", "") | (":load", "") => "Provide a file path after :save or :load.".into(),
                (":save", path) => match write_save(Path::new(path), &engine) {
                    Ok(()) => format!("Saved to {path}."),
                    Err(error) => error.to_string(),
                },
                (":load", path) => match read_save(Path::new(path)) {
                    Ok(save) => match engine.restore(save) {
                        Ok(_) => format!("Loaded {path}."),
                        Err(error) => format!("{path}: cannot restore: {error}"),
                    },
                    Err(error) => error.to_string(),
                },
                _ => "Unknown session command. Use :save PATH, :load PATH, or :quit.".into(),
            };
            renderer.render(&[PresentationCommand::Text(message)])?;
            continue;
        }
        let Some(action) = adapter.resolve(&input) else {
            renderer.render(&[PresentationCommand::Text(
                "Enter a displayed choice number.".into(),
            )])?;
            continue;
        };
        if let Err(error) = engine.apply(&action) {
            renderer.render(&[PresentationCommand::Text(format!(
                "Choice rejected: {error}"
            ))])?;
        }
    }
}
