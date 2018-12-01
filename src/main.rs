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
        .version("0.1")
        .author("Hugo Laloge")
        .subcommand(clap::SubCommand::with_name("editor").about("Launch the level editor"))
        .get_matches();

    if matches.subcommand_matches("edit").is_some() {
        println!("No game editor for now");
        Ok(())
    } else {
        bobo_is_you::start_game()
    }
}
