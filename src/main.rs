extern crate amethyst;
extern crate bobo_is_you;
extern crate clap;

mod logger;

use std::env;

fn main() -> Result<(), amethyst::Error> {
    if env::var("RUST_LOG") == Err(env::VarError::NotPresent) {
        env::set_var("RUST_LOG", "debug,gfx_device_gl=warn,amethyst_assets=warn");
    }

    logger::start_logger(Default::default());

    let matches = clap::App::new("Bobo Is You")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(clap::SubCommand::with_name("editor").about("Launch the level editor"))
        .get_matches();

    if matches.subcommand_matches("editor").is_some() {
        bobo_is_you::editor::start_level_editor()
    } else {
        bobo_is_you::start_game()
    }
}
