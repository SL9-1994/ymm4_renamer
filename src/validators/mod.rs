pub mod kitsune_yukkuri;
use std::path::PathBuf;

use kitsune_yukkuri::KitsuneYukkuri;
use log::debug;

use crate::{args::RenameOption, error::MultipleErrors};

// validator factory pattern
pub struct ValidatorFactory;

impl ValidatorFactory {
    pub fn create_validator(
        rename_option: &RenameOption,
        input_folder: PathBuf,
    ) -> Box<dyn Validator> {
        debug!("Creating validator for option: {:?}", rename_option);

        match rename_option {
            RenameOption::KitsuneYukkuri => Box::new(KitsuneYukkuri { input_folder }),
            //RenameOption::Other => Box::new(Other),
        }
    }
}

pub trait Validator {
    fn validate(&self) -> Result<(), MultipleErrors>;
}
