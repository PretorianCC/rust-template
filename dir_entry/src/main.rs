use std::fs;

fn main() {
   
    // Возвращает путь к файлу или каталогу.
    let entries = fs::read_dir("..").unwrap();
    println!("path");
    for entry in entries {
        let path = entry.unwrap().path(); // path "..\\Cargo.lock" ...
        println!("{:?}", path);
    }

    // Возвращает методанные к файла или каталога.
    let entries = fs::read_dir("..").unwrap();
    println!("metadata");
    for entry in entries {
        let metadata = entry.unwrap().metadata();
        println!("{:?}", metadata); // metadata Ok(Metadata {...}) ...
    }

    // Возвращает тип для файла или каталога.
    let entries = fs::read_dir("..").unwrap();
    println!("file_type");
    for entry in entries {
        let file_type = entry.unwrap().file_type();
        println!("{:?}", file_type); // file_type Ok(FileType(FileType { attributes: 32, reparse_tag: 0 })) ...
    }

    // Возвращает имя файла или каталога.
    let entries = fs::read_dir("..").unwrap();
    println!("file_type");
    for entry in entries {
        let file_type = entry.unwrap().file_name();
        println!("{:?}", file_type); // file_type "Cargo.lock" ...
    }

}
