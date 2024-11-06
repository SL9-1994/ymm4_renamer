use args::Args;
use log::{debug, error, info, LevelFilter};
use renamers::RenamerFactory;
use validators::ValidatorFactory;

mod args;
mod error;
mod logging;
mod renamers;
mod validators;

fn main() {
    let args = Args::new();
    let log_level: LevelFilter = args.log_level.clone().into();
    logging::init_logger(log_level);

    debug!("Parsed arguments: {:?}", args);

    let validator =
        ValidatorFactory::create_validator(&args.rename_option, args.input_folder.to_path_buf());

    match validator.validate() {
        Ok(()) => {
            info!("Validation succeeded.");
            let renamer = RenamerFactory::create_renamer(
                &args.rename_option,
                args.input_folder.to_path_buf(),
            );

            match renamer.rename() {
                Ok(()) => {
                    info!("Rename succeeded.");
                    info!("All process succeeded!!");
                    print_ascii();
                }
                Err(e) => {
                    error!("{e}")
                }
            }
        }
        Err(e) => {
            error!("{e}");
        }
    }
}

fn print_ascii() {
    println!(
        r#"
 ________  ________  _____ ______   ________  ___       _______  _________  _______   ________     
|\   ____\|\   __  \|\   _ \  _   \|\   __  \|\  \     |\  ___ \|\___   ___\\  ___ \ |\   ___ \    
\ \  \___|\ \  \|\  \ \  \\\__\ \  \ \  \|\  \ \  \    \ \   __/\|___ \  \_\ \   __/|\ \  \_|\ \   
 \ \  \    \ \  \\\  \ \  \\|__| \  \ \   ____\ \  \    \ \  \_|/__  \ \  \ \ \  \_|/_\ \  \ \\ \  
  \ \  \____\ \  \\\  \ \  \    \ \  \ \  \___|\ \  \____\ \  \_|\ \  \ \  \ \ \  \_|\ \ \  \_\\ \ 
   \ \_______\ \_______\ \__\    \ \__\ \__\    \ \_______\ \_______\  \ \__\ \ \_______\ \_______\
    \|_______|\|_______|\|__|     \|__|\|__|     \|_______|\|_______|   \|__|  \|_______|\|_______|                                                               
        "#
    )
}
