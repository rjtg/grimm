use grimm_adapters::{InputAdapter, OutputAdapter, TextAdapter};
use grimm_cli_renderer::CliRenderer;
use grimm_engine::Engine;
use grimm_presentation::PresentationCommand;
use std::io;

fn main() -> io::Result<()> {
    let mut engine = Engine::default();
    let mut adapter = TextAdapter::default();
    let mut renderer = CliRenderer::new(io::stdin().lock(), io::stdout().lock());
    let mut output = engine.output();
    loop {
        renderer.render(&adapter.present(&output))?;
        if output.offers.is_empty() {
            return Ok(());
        }
        let Some(input) = renderer.read_input()? else {
            return Ok(());
        };
        let Some(action) = adapter.resolve(&input) else {
            renderer.render(&[PresentationCommand::Text(
                "Enter a displayed choice number.".into(),
            )])?;
            continue;
        };
        match engine.apply(&action) {
            Ok(next) => output = next,
            Err(_) => output = engine.output(),
        }
    }
}
