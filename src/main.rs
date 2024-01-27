use std::{fs, io};


fn main() {
    let path: &'static str = "C:\\Users\\nilson.martins\\Downloads\\";

    match get_files(path) {
        Ok(file_names) => {
           for file_name in file_names {
                if file_name.contains(".rdp"){
                    let mut owned_path: String = path.to_owned();
                    let borrowed_filename: &str = file_name.as_str();
                    owned_path.push_str(borrowed_filename);
                    let static_path: &str = &owned_path[..];
                    println!("File Path: {}", static_path);
                    delete_file(static_path);
                }
           }
        }
        Err(e) =>   println!("Error: {}", e),
    }
}

fn get_files(path: &str) -> io::Result<Vec<String>>{

        // Get a list of all entries in the folder
        let entries: fs::ReadDir = fs::read_dir(path)?;

        // Extract the filenames from the directory entries and store them in a vector
        let file_names: Vec<String> = entries
        .filter_map(|entry: Result<fs::DirEntry, io::Error>| {
            let path = entry.ok()?.path();
            if path.is_file(){
                path.file_name()?.to_str().map(|s| s.to_lowercase())
            }else{
                None
            }
        }).collect();

        Ok(file_names)
}

fn delete_file(path: &str){
    fs::remove_file(path)
    .expect("Delete failed!");
}