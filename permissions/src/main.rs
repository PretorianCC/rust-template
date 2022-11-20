use std::fs;

fn main() {

    // Файл только на чтение?
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    let permissions = metadata.permissions();
    println!("readonly {:?}", permissions.readonly()); // readonly false

    // Изменяет флаг только для чтения.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    let mut permissions = metadata.permissions();
    println!("set_readonly {:?}", permissions.set_readonly(false));  // set_readonly ()  
}
