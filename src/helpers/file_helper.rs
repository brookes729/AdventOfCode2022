use std::{fs, path::Path};

pub fn read_file_to_string(file_name: &str) -> String {
    let file_path_buffer = std::env::current_dir().unwrap();
    let file_path = file_path_buffer.to_str().unwrap();

    let path = Path::new(&file_path).join("inputs").join(file_name);
    let contents: String;

    //println!("In path {}", path.to_string_lossy());
    if path.exists() {
        let result = fs::read_to_string(&path);
        match result {
            Ok(output) => contents = output,
            Err(error) => {
                println!("Error Reading file {error}");
                contents = String::from("")
            }
        }
    } else {
        contents = String::from("")
    }

    //println!("With text:\n{contents}");
    contents
}
