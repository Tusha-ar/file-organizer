use std::{fs::{self}, io::ErrorKind, path::PathBuf};

fn main() {
    let mut directory_to_organize = String::new();
    println!("Enter a dir. you want to organise");
    loop {
        match std::io::stdin().read_line(&mut directory_to_organize) {
            Ok(_) => break,
            Err(_) => {
                println!("Enter a valid input");
                continue;
            }
        }
    }
    let directory_to_organize = directory_to_organize.trim();
    let paths = fs::read_dir(directory_to_organize).unwrap();
    for path in paths {
        let path = path.unwrap().path();

        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext = ext.to_str().unwrap().to_lowercase();
                let folder_name = match ext.as_str() {
                    "jpg" | "jpeg" | "png" | "svg" => "Images",
                    "pdf" | "docx" | "txt" => "Documents",
                    "mp4" | "mov" | "avi" => "Videos",
                    "rs" | "py" | "cpp" => "Code",
                    _ => "Others",
                };
                let new_folder_path = format!("{}/{}", directory_to_organize, folder_name);
                println!("Creating New dir {}", new_folder_path);
                match fs::create_dir(new_folder_path) {
                    Ok(_) => {
                        write_to_folder(&path, directory_to_organize, &folder_name);
                    },
                    Err(err) => {
                        if err.kind() == ErrorKind::AlreadyExists {
                            write_to_folder(&path, directory_to_organize, &folder_name);
                        }
                    }
                };
            }
        }
    }
    println!("Files organized successfully!")

}


fn write_to_folder(path: &PathBuf, directory_to_organize: &str, folder_name: &str) {
    let file_name = path.file_name().unwrap();
    let new_file_path = format!("{}/{}/{}", directory_to_organize, folder_name, file_name.to_str().unwrap());
    println!("Writing the file: {}", new_file_path);
    match fs::rename(&path, new_file_path) {
        Ok(_) => {},
        Err(err) => {
            println!("error while writing the file File: {:?}, err: {}", path, err);
        }
    };
}