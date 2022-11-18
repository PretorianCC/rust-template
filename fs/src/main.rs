use std::fs;

fn main() {
    
    // Абсолютный путь.
    let path = "..";
    println!("canonicalize {:?}", fs::canonicalize(path)); // canonicalize Ok("\\\\?\\C:\\Projects\\Rust\\rust-template\\fs")

    // Копирование файла.
    let path_in = "main.rs";
    let path_out = "main.tmp";
    let result = fs::copy(path_in, path_out);
    println!("copy {:?}", result); // copy Ok(403)

    // Создает каталог.
    let path = "./tmp";
    let result = fs::create_dir(path);
    println!("create_dir {:?}", result); // create_dir Ok(())

    // Удаляет пустой каталог.
    let path = "./tmp";
    let result = fs::remove_dir(path);
    println!("remove_dir {:?}", result); // remove_dir Ok(())

    // Рекурсивно создает каталоги.
    let path = "./tmp/tmp";
    let result = fs::create_dir_all(path);
    println!("create_dir_all {:?}", result); // create_dir_all Ok(())

    // Переименовывает файл или каталог.
    let path_in = "./tmp/tmp";
    let path_out = "./tmp/tmpl";
    let result = fs::rename(path_in, path_out);
    println!("rename {:?}", result); // rename Ok(())

    // Удаляет каталог и все его содержимое.
    let path = "./tmp";
    let result = fs::remove_dir_all(path);
    println!("remove_dir_all {:?}", result); // remove_dir_all Ok(())

    // Создает ссылку на файл.
    let path_in = "main.rs";
    let path_out = "1.tmp";
    let result = fs::hard_link(path_in, path_out);
    println!("hard_link {:?}", result); // hard_link Ok(())

    // Удаляет файл.
    let path = "1.tmp";
    let result = fs::remove_file(path);
    println!("remove_file {:?}", result); // remove_file Ok(())

    // Метаданные файла.
    let path = "main.rs";
    let result = fs::metadata(path);
    println!("metadata {:?}", result); // metadata Ok(Metadata { ... })

    // Прочитать файл.
    let path = "main.rs";
    let data = fs::read(path).unwrap();
    let text: String = String::from_utf8_lossy(&data).parse().unwrap();
    println!("read {}", text); // read ...
    
    // Итератор по записям в директории.
    let path = "./";
    let data = fs::read_dir(path).unwrap();
    for item in data {
        println!("{:?}", item); // Ok(DirEntry("..."))
    }

    // Возвращает файл по ссылке.
    let path = "main.tmp";
    let file = fs::read_link(path);
    println!("read_link {:?}", file); // read_link Err(Os { code: 4390, kind: Uncategorized, message: "..." })

    // Прочитать файл в строку.
    let path = "main.rs";
    let data = fs::read_to_string(path).unwrap();
    println!("read_to_string {}", data); // read_to_string ...

    // Изменяет разрешения файла или каталога.
    let path = "../Cargo.lock";
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_readonly(false);
    let result = fs::set_permissions(path, perms);
    println!("set_permissions {:?}", result); // set_permissions Ok(())

    // Метаданные ссылки на файл.
    let path = "main.rs";
    let result = fs::symlink_metadata(path);
    println!("symlink_metadata {:?}", result); // symlink_metadata Ok(Metadata { ... })
    
    // Записать в файл строку.
    let path = "1.tmp";
    let result = fs::write(path, "contents");
    println!("write {:?}", result); // write Ok(())
    
}
