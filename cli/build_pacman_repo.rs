use pacman_repo_builder::app::App;
use std::process::ExitCode;

fn main() -> ExitCode {
    App::from_env().run()
}
