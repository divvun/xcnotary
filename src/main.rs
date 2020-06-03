mod notarize;
mod precheck;
mod util;

use console::Style;
use std::error::Error;
use util::cli::Args;
use notarize::Password;

fn main() {
    run().unwrap_or_else(|err| {
        eprintln!("\n{}", err);
        std::process::exit(1);
    });
}

fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = util::cli::parse();

    let emphasized = Style::new().white().bold();
    println!("{}\n", emphasized.apply_to("Processing..."),);

    match args {
        Args::Precheck { input_path } => {
            let path_type = util::input_path::identify_path_type(&input_path)?;
            precheck::run(&input_path, &path_type, true)?;
        }
        Args::Notarize {
            developer_account,
            password,
            password_env,
            password_keychain_item,
            input_path,
            provider,
            override_path_type,
        } => {
            let (path_type, bundle_id) =
                util::input_path::path_info(&input_path, override_path_type)?;

            let password = if let Some(p) = password {
                Password::Literal(p)
            } else if let Some(p) = password_env {
                Password::Env(p)
            } else if let Some(p) = password_keychain_item {
                Password::Keychain(p)
            } else {
                panic!("You must provide a password in some form.");
            };

            precheck::run(&input_path, &path_type, false)?;
            notarize::run(
                input_path,
                path_type,
                bundle_id,
                developer_account,
                password,
                provider,
            )?;
        }
    }

    Ok(())
}
