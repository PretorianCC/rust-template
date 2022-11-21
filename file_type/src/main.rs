use std::fs;

fn main() {
    
    // Это директория?
    let metadata = fs::metadata("main.rs").unwrap();
    let file_type = metadata.file_type();
    println!("is_dir {:?}", file_type.is_dir()); // is_dir false

    // Это файл?
    let metadata = fs::metadata("main.rs").unwrap();
    let file_type = metadata.file_type();
    println!("is_file {:?}", file_type.is_file()); // is_file true

    // Это символьная ссылка?
    let metadata = fs::metadata("main.rs").unwrap();
    let file_type = metadata.file_type();
    println!("is_symlink {:?}", file_type.is_symlink()); // is_symlink false

}
