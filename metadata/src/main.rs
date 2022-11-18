use std::fs;

fn main() {
    
    // Время последнего доступа к метаданным.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("accessed {:?}", metadata.accessed()); // accessed Ok(SystemTime { intervals: 133132474556546965 })

    // Время создания.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("created {:?}", metadata.created()); // created Ok(SystemTime { intervals: 133132441936147534 })
    

    // Возвращает тип файла.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("file_type {:?}", metadata.file_type()); // file_type FileType(FileType { attributes: 32, reparse_tag: 0 })

    // Это каталог?
    let path = "..";
    let metadata = fs::metadata(path).unwrap();
    println!("is_dir {:?}", metadata.is_dir()); // is_dir true

    // Это файл?
    let path = "..";
    let metadata = fs::metadata(path).unwrap();
    println!("is_file {:?}", metadata.is_file()); // is_file false

    // Это ссылка на файл?
    let path = "..";
    let metadata = fs::metadata(path).unwrap();
    println!("is_symlink {:?}", metadata.is_symlink()); // is_symlink false

    // Возвращает размер файла (в байтах).
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("len {:?}", metadata.len()); // len 1547

    // Возвращает время последнего изменения.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("modified {:?}", metadata.modified()); // modified Ok(SystemTime { intervals: 133132503893722499 })

    // Возвращает разрешения.
    let path = "main.rs";
    let metadata = fs::metadata(path).unwrap();
    println!("permissions {:?}", metadata.permissions()); // permissions Permissions(FilePermissions { attrs: 32 })

}
