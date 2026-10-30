use grimm_adapters::{InputAdapter, OutputAdapter, TextAdapter};
use grimm_cli::{load_story, read_save, write_save};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::{Engine, PlayabilityLimits};
use grimm_presentation::PresentationCommand;
use std::{
    env,
    io::{self, IsTerminal},
    path::Path,
    process::ExitCode,
};

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
    if args.first().is_some_and(|command| command == "play-scene") {
        return grimm_cli::scene_runtime::run(&args[1..]);
    }
    if let [command, path] = args.as_slice()
        && command == "check-scene"
    {
        return grimm_cli::scene_runtime::check(Path::new(path));
    }
    if let [command, story, rest @ ..] = args.as_slice()
        && command == "check"
    {
        let mut limits = PlayabilityLimits::default();
        let (options, remainder) = rest.as_chunks::<2>();
        for pair in options {
            let value = pair[1]
                .to_str()
                .and_then(|value| value.parse::<usize>().ok())
                .ok_or("Check budgets must be nonnegative integers")?;
            if pair[0] == "--max-states" {
                limits.max_states = value;
            } else if pair[0] == "--max-transitions" {
                limits.max_transitions = value;
            } else {
                return Err(
                    "Unknown check option; use --max-states N or --max-transitions N".into(),
                );
            }
        }
        if !remainder.is_empty() {
            return Err("Missing check budget value".into());
        }
        return check_story(Path::new(story), limits);
    }
    let (title, mut engine) = match args.as_slice() {
        [] => (None, Engine::default()),
        [help] if help == "--help" || help == "-h" => {
            println!(
                "Usage: grimm-cli [play STORY.yaml [--load SAVE.json]]\n       grimm-cli check STORY.yaml [--max-states N] [--max-transitions N]\n       grimm-cli play-scene STORY.yaml TEXT.yaml [--manual-clock] [--load SAVE.json]\n       grimm-cli check-scene STORY.yaml\nTerminal: Esc opens the session menu (1 Continue, 2 Save, 3 Load, 4 Quit).\nPiped session commands: :save PATH, :load PATH, :quit"
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
    if engine.save().temporal.is_some()
        || engine.save().schema_version == 3
        || (io::stdin().is_terminal() && io::stdout().is_terminal())
    {
        return grimm_cli::runtime::play(title.as_deref().unwrap_or("GRIMM"), engine);
    }
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
            engine.validate_current_state().map_err(|error| format!("Story blocked: {error}. Run 'grimm-cli check STORY.yaml' for authoring diagnostics; this is not an authored ending."))?;
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

fn check_story(path: &Path, limits: PlayabilityLimits) -> Result<(), Box<dyn std::error::Error>> {
    let (title, engine) = load_story(path)?;
    let report = engine.check_playability(limits);
    println!(
        "{title}: {} states checked, {} choices inspected.",
        report.states_checked, report.transitions_checked
    );
    for deadlock in &report.deadlocks {
        println!(
            "Nonterminal deadlock at node '{}': {}",
            deadlock.node_id, deadlock.reason
        );
        for step in &deadlock.path {
            println!("  node '{}' -> choice '{}'", step.node_id, step.choice_id);
        }
    }
    println!(
        "Authored endings observed: {}",
        report
            .endings
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(", ")
    );
    if report.truncated {
        return Err(format!("Playability check inconclusive: budget exhausted ({} states / {} choices). Increase --max-states or --max-transitions; absence of findings is not validation.", limits.max_states, limits.max_transitions).into());
    }
    if !report.deadlocks.is_empty() {
        return Err(format!(
            "{}: playability check found {} nonterminal deadlock(s)",
            path.display(),
            report.deadlocks.len()
        )
        .into());
    }
    println!(
        "No nonterminal deadlocks found in this bounded check; this is not a proof of eventual completion."
    );
    Ok(())
}
