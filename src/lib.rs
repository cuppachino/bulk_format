use std::path::PathBuf;

pub fn prompt_bool(prompt: &str) -> bool {
    loop {
        println!("{}", prompt);
        println!("[y/N]");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input.");
        match input.trim().to_lowercase().as_str() {
            "y" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => {
                continue;
            }
        }
    }
}

/// Checks if the target file exists, and if it does, prompts the user if they want to overwrite it.
/// If they do not want to overwrite it, a new target file name is generated in a loop until a unique name is found.
pub fn safely_target_file(target: &str) -> String {
    let mut target = target.to_string();
    let mut target_path = PathBuf::from(target.as_str());
    let mut target_exists = target_path.exists();
    while target_exists {
        let should_overwrite = prompt_bool(
            &format!("The target file \"{}\" already exists. Do you want to overwrite it?", target)
        );
        if !should_overwrite {
            let mut target_name = target_path
                .file_stem()
                .expect("Failed to get file stem.")
                .to_string_lossy()
                .to_string();
            let target_ext = target_path
                .extension()
                .expect("Failed to get file extension.")
                .to_string_lossy()
                .to_string();
            let target_dir = target_path
                .parent()
                .expect("Failed to get parent directory.")
                .to_string_lossy()
                .to_string();
            let mut target_index = 1;
            while target_exists {
                target_name = format!("{}_{}", target_name, target_index);
                target = format!("{}/{}.{}", target_dir, target_name, target_ext);
                target_path = PathBuf::from(target.as_str());
                target_exists = target_path.exists();
                target_index += 1;
            }
        }
    }
    target
}
