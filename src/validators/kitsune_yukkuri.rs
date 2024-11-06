use std::path::PathBuf;

use log::debug;

use crate::error::{Errors, MultipleErrors, ValidationError};

use super::Validator;

pub struct KitsuneYukkuri {
    pub input_folder: PathBuf,
}

impl Validator for KitsuneYukkuri {
    fn validate(&self) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();

        if let Err(e) = self.validate_folder() {
            errors.extend(e);
        } else if let Err(e) = self.validate_sub_folder() {
            errors.extend(e);
        }

        if errors.is_empty() {
            debug!("Succeeded in validating folder: {:?}", self.input_folder);
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }
}

impl KitsuneYukkuri {
    fn validate_folder(&self) -> Result<(), MultipleErrors> {
        let input_folder = self.input_folder.to_path_buf();
        debug!("Validating folder: {:?}", self.input_folder);
        let mut errors = Vec::new();

        match std::fs::metadata(&self.input_folder) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    errors.push(ValidationError::InputPathIsNotDirectory(input_folder).into());
                }
            }

            Err(e) => errors.push(Errors::from_io_error(e, input_folder)),
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }

    fn validate_sub_folder(&self) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();
        let subfolders = ["口", "目"];

        for subfolder in &subfolders {
            debug!("Checking for subfolder: {:?}", subfolder);
            let folder_path = self.input_folder.join(subfolder);

            // サブフォルダ内に少なくとも1つの PNG ファイルがあるか確認
            let read_dir_result = std::fs::read_dir(&folder_path);
            match read_dir_result {
                Ok(entries) => {
                    let entries_vec: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
                    debug!(
                        "Found {} entries in directory: {:?}",
                        entries_vec.len(),
                        folder_path
                    );

                    let has_png = entries_vec.iter().any(|entry| {
                        entry.path().extension().and_then(|ext| ext.to_str()) == Some("png")
                    });

                    if !has_png {
                        errors.push(ValidationError::NoPngFilesInSubDir(folder_path).into());
                    }
                }
                Err(e) => {
                    errors.push(Errors::from_io_error(e, folder_path.clone()));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }
}
