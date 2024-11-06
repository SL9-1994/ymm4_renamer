use std::{collections::HashMap, fs, path::PathBuf};

use log::{debug, info};
use regex::Regex;

use crate::error::{Errors, MultipleErrors, RenameError};

use super::Renamer;

pub struct KitsuneYukkuri {
    pub input_folder: PathBuf,
}

impl Renamer for KitsuneYukkuri {
    fn rename(&self) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();

        let mouth_subdir = self.get_mouth_directory();

        match self.rename_mouse_folder(mouth_subdir.clone()) {
            Ok(()) => {
                info!("Succeeded in rename of {:?}", mouth_subdir)
            }
            Err(e) => {
                errors.extend(e);
            }
        }

        let eye_subdir = self.get_eye_directory();

        match self.rename_eye_folder(eye_subdir.clone()) {
            Ok(()) => {
                info!("Secceeded in rename of {:?}", eye_subdir)
            }
            Err(e) => {
                errors.extend(e);
            }
        }

        let face_subdir = self.get_face_directory();
        match self.rename_face_folder(face_subdir.clone()) {
            Ok(()) => {
                info!("Succeeded in rename of {:?}", face_subdir)
            }
            Err(e) => {
                errors.extend(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }
}

impl KitsuneYukkuri {
    fn get_mouth_directory(&self) -> PathBuf {
        self.input_folder.join("口")
    }

    fn get_eye_directory(&self) -> PathBuf {
        self.input_folder.join("目")
    }

    fn get_face_directory(&self) -> PathBuf {
        self.input_folder.join("顔")
    }

    fn rename_mouse_folder(&self, mouth_subdir: PathBuf) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();

        let pattern = r"^(\d+)([a-z]?)(-\d+)?\.png$";
        let re = match Regex::new(pattern) {
            Ok(regex) => regex,
            Err(e) => {
                debug!("Error creating regex: {}", e);
                errors.push(RenameError::RegexError(e).into());
                return Err(MultipleErrors::new(errors));
            }
        };

        let mut file_groups: HashMap<String, Vec<(String, PathBuf)>> = HashMap::new();

        let entries = match fs::read_dir(mouth_subdir.clone()) {
            Ok(entries) => entries,
            Err(e) => {
                debug!("Failed to read directory: {}", e);
                errors.push(Errors::from_io_error(e, mouth_subdir.clone()));
                return Err(MultipleErrors::new(errors));
            }
        };

        for entry in entries {
            // 各エントリの取得
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    debug!("Failed to retrieve entry: {}", e);
                    errors.push(Errors::from_io_error(e, mouth_subdir.clone()));
                    continue;
                }
            };

            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            // 正規表現でファイル名をパース
            if let Some(captures) = re.captures(file_name) {
                let number = captures[1].to_string();
                let letter = captures[2].to_string();

                // 各番号グループ（例: 00, 01）ごとにファイルを分類
                file_groups.entry(number).or_default().push((letter, path));
            }
        }

        // 各グループごとに処理
        for (number, mut files) in file_groups {
            // アルファベット順にソート
            files.sort_by_key(|(letter, _)| letter.clone());

            // 最後のファイルは「00.png」のようにリネームし、それ以外は中間フレームとする
            for (index, (_letter, path)) in files.iter().enumerate() {
                let new_file_name = if index == files.len() - 1 {
                    format!("{}.png", number) // 最後のファイル
                } else {
                    format!("{}.{}.png", number, index) // 中間フレーム
                };

                let new_path = mouth_subdir.join(new_file_name);

                if let Err(e) = fs::rename(path, &new_path) {
                    errors.push(Errors::from_io_error(e, mouth_subdir.clone()));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }

    fn rename_eye_folder(&self, eye_subdir: PathBuf) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();

        let pattern = r"^(\d+)([a-z]?)(-\d+)?\.png$";
        let re = match Regex::new(pattern) {
            Ok(regex) => regex,
            Err(e) => {
                debug!("Error creating regex: {}", e);
                errors.push(RenameError::RegexError(e).into());
                return Err(MultipleErrors::new(errors));
            }
        };

        // フォルダー内の全てのファイルを取得し、グループごとにファイル名を整理
        let mut file_groups: HashMap<String, Vec<(String, String, PathBuf)>> = HashMap::new();

        let entries = match fs::read_dir(eye_subdir.clone()) {
            Ok(entries) => entries,
            Err(e) => {
                debug!("Failed to read directory: {}", e);
                errors.push(Errors::from_io_error(e, eye_subdir.clone()));
                return Err(MultipleErrors::new(errors));
            }
        };

        for entry in entries {
            // 各エントリの取得
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    debug!("Failed to retrieve entry: {}", e);
                    errors.push(Errors::from_io_error(e, eye_subdir.clone()));
                    continue; // エラーが発生した場合はスキップ
                }
            };

            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            // 正規表現でファイル名をパース
            if let Some(captures) = re.captures(file_name) {
                let number = captures[1].to_string();
                let letter = captures
                    .get(2)
                    .map_or("".to_string(), |m| m.as_str().to_string());
                let suffix = captures
                    .get(3)
                    .map_or("".to_string(), |m| m.as_str().to_string());

                // 各番号グループ（例: 00, 01）ごとにファイルを分類
                file_groups
                    .entry(number.clone())
                    .or_default()
                    .push((letter, suffix, path)); // suffixを保持
            }
        }

        for (number, mut files) in file_groups {
            files.sort_by_key(|(letter, _, _)| letter.clone());

            let letter_count = files.len();

            for (index, (letter, _suffix, path)) in files.iter().enumerate() {
                let new_file_name = if letter.is_empty() {
                    format!("{}.png", number)
                } else {
                    let new_index = if index == letter_count - 1 {
                        0
                    } else {
                        letter_count - 1 - index
                    };

                    format!("{}.{}.png", number, new_index)
                };

                let new_path = eye_subdir.join(new_file_name);

                if let Err(e) = fs::rename(path, &new_path) {
                    errors.push(Errors::from_io_error(e, eye_subdir.clone()));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }

    fn rename_face_folder(&self, face_dir_path: PathBuf) -> Result<(), MultipleErrors> {
        let mut errors = Vec::new();

        let path = face_dir_path;
        let new_path = self.input_folder.join("顔色");

        if let Err(e) = fs::rename(&path, &new_path) {
            errors.push(Errors::from_io_error(e, path));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(MultipleErrors::new(errors))
        }
    }
}
