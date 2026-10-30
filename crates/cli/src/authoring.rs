//! YAML source locations and file/CLI composition for renderer-free tooling.
use grimm_semantic::{authoring::*, scene::SceneStory};
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};
use std::{collections::BTreeMap, ffi::OsString, fmt, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Parse with the same strict SceneStory codec used by play-scene.
pub fn inspect(file: &str, yaml: &str) -> Report {
    let mut report = match serde_yaml_ng::from_str::<SceneStory>(yaml) {
        Ok(story) => grimm_engine::scene::authoring::analyze(story),
        Err(error) => {
            return Report {
                schema_version: 1,
                index: None,
                diagnostics: vec![Diagnostic {
                    severity: Severity::Error,
                    code: "invalid_yaml".into(),
                    message: error.to_string(),
                    source: Source {
                        file: file.into(),
                        path: String::new(),
                        line: error.location().map(|p| p.line()),
                        column: error.location().map(|p| p.column()),
                    },
                    related: vec![],
                    suggestion: "Correct the YAML/schema error, then run check-scene again.".into(),
                }],
            };
        }
    };
    let mut cache = BTreeMap::new();
    let mut locate = |source: &mut Source| {
        source.file = file.into();
        let location = cache.entry(source.path.clone()).or_insert_with(|| {
            let mut path = source.path.as_str();
            loop {
                if let Some(position) = position(yaml, path) {
                    break Some(position);
                }
                let Some((parent, _)) = path.rsplit_once('/') else {
                    break None;
                };
                path = parent;
            }
        });
        if let Some((line, column)) = location {
            source.line = Some(*line);
            source.column = Some(*column);
        }
    };
    for d in &mut report.diagnostics {
        locate(&mut d.source);
        for s in &mut d.related {
            locate(s);
        }
    }
    if let Some(index) = &mut report.index {
        for c in &mut index.contexts {
            locate(&mut c.source);
            for a in &mut c.actions {
                locate(&mut a.source);
                for source in &mut a.uses {
                    locate(source);
                }
            }
            for cue in &mut c.cues {
                locate(&mut cue.source);
            }
            for n in &mut c.narratives {
                for s in &mut n.uses {
                    locate(s);
                }
            }
            for t in &mut c.transitions {
                locate(&mut t.source);
            }
        }
    }
    report
}

pub fn run(index: bool, args: &[OsString]) -> Result<()> {
    let (path, json) = match args {
        [path] => (Path::new(path), false),
        [path, flag] if flag == "--json" => (Path::new(path), true),
        _ => {
            return Err(
                "Usage: grimm-cli check-scene STORY.yaml [--json] | index-scene STORY.yaml".into(),
            );
        }
    };
    let yaml = fs::read_to_string(path).map_err(|error| format!("{}: {error}", path.display()))?;
    let report = inspect(&path.display().to_string(), &yaml);
    if index && !report.has_errors() {
        println!(
            "{}",
            serde_json::to_string_pretty(report.index.as_ref().expect("validated index"))?
        );
    } else if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        for diagnostic in &report.diagnostics {
            eprintln!("{}", display(diagnostic));
        }
        if !report.has_errors() {
            println!(
                "{}: valid scene story; structural exits checked, state-dependent solvability is not proven.",
                path.display()
            );
        }
    }
    if report.has_errors() {
        return Err("Scene authoring validation failed".into());
    }
    Ok(())
}

fn display(d: &Diagnostic) -> String {
    let source = |s: &Source| {
        format!(
            "{}:{}:{} ({})",
            s.file,
            s.line.map_or_else(|| "?".into(), |n| n.to_string()),
            s.column.map_or_else(|| "?".into(), |n| n.to_string()),
            s.path
        )
    };
    let severity = match d.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
        Severity::Info => "info",
    };
    let mut result = format!(
        "{}: {severity}[{}]: {}\n  help: {}",
        source(&d.source),
        d.code,
        d.message,
        d.suggestion
    );
    for related in &d.related {
        result.push_str(&format!("\n  related declaration: {}", source(related)));
    }
    result
}

/// Ask the existing YAML deserializer to visit a JSON Pointer. Its normal error
/// mark supplies the target node location, avoiding a second YAML parser or
/// brittle line matching (including for flow mappings, quoting and comments).
fn position(yaml: &str, path: &str) -> Option<(usize, usize)> {
    struct Stop;
    impl<'de> Visitor<'de> for Stop {
        type Value = ();
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("GRIMM source location marker")
        }
    }
    struct Walk<'a>(&'a [String]);
    impl<'de> DeserializeSeed<'de> for Walk<'_> {
        type Value = ();
        fn deserialize<D: de::Deserializer<'de>>(
            self,
            deserializer: D,
        ) -> std::result::Result<(), D::Error> {
            if self.0.is_empty() {
                deserializer.deserialize_any(Stop)
            } else {
                deserializer.deserialize_any(self)
            }
        }
    }
    impl<'de> Visitor<'de> for Walk<'_> {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an authored source path")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> std::result::Result<(), M::Error> {
            while let Some(key) = map.next_key::<String>()? {
                if key == self.0[0] {
                    return map.next_value_seed(Walk(&self.0[1..]));
                }
                map.next_value::<IgnoredAny>()?;
            }
            Err(de::Error::custom("source path not found"))
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> std::result::Result<(), S::Error> {
            let index: usize = self.0[0].parse().map_err(de::Error::custom)?;
            for _ in 0..index {
                seq.next_element::<IgnoredAny>()?;
            }
            seq.next_element_seed(Walk(&self.0[1..]))?
                .ok_or_else(|| de::Error::custom("source path not found"))
        }
    }
    let path: Vec<_> = path
        .strip_prefix('/')
        .unwrap_or(path)
        .split('/')
        .filter(|p| !p.is_empty())
        .map(|p| p.replace("~1", "/").replace("~0", "~"))
        .collect();
    let error = Walk(&path)
        .deserialize(serde_yaml_ng::Deserializer::from_str(yaml))
        .err()?;
    if !error.to_string().contains("GRIMM source location marker") {
        return None;
    }
    error.location().map(|p| (p.line(), p.column()))
}
