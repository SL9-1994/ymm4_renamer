use std::path::PathBuf;

use kitsune_yukkuri::KitsuneYukkuri;
use log::debug;

use crate::{args::RenameOption, error::MultipleErrors};

pub mod kitsune_yukkuri;

// validator factory pattern
pub struct RenamerFactory;

impl RenamerFactory {
    pub fn create_renamer(rename_option: &RenameOption, input_folder: PathBuf) -> Box<dyn Renamer> {
        debug!("Creating Renamer for option: {:?}", rename_option);
        match rename_option {
            RenameOption::KitsuneYukkuri => Box::new(KitsuneYukkuri { input_folder }),
            //RenameOption::Other => Box::new(Other),
        }
    }
}

pub trait Renamer {
    fn rename(&self) -> Result<(), MultipleErrors>;
}
