use std::fs::OpenOptions;

fn main() {

    // Создает пустой новый набор параметров, готовых к настройке. Для всех параметров изначально задано значение false.
    let mut options = OpenOptions::new();
    let file = options.read(true).open("main.rs");
    println!("OpenOptions::new {:?} {:?}", options, file); // OpenOptions::new OpenOptions(OpenOptions { read: true, write: false, append: false, truncate: false, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Задает параметр доступа для чтения. true, файл должен быть доступен для чтения при открытии.
    let mut options = OpenOptions::new();
    options.read(true);
    println!("read {:?}", options); // read OpenOptions(OpenOptions { read: true, write: false, append: false, truncate: false, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Задает параметр для доступа на запись. true, файл должен быть доступен для записи, если он открыт.
    let mut options = OpenOptions::new();
    options.write(true);
    println!("write {:?}", options); // write OpenOptions(OpenOptions { read: false, write: true, append: false, truncate: false, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Задает параметр для режима добавления. true, запись будет добавлена в файл вместо перезаписи предыдущего содержимого.
    let mut options = OpenOptions::new();
    options.append(true);
    println!("append {:?}", options); // append OpenOptions(OpenOptions { read: false, write: false, append: true, truncate: false, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Задает параметр для создания нового файла или открытия его, если он уже существует.
    let mut options = OpenOptions::new();
    options.create(true);
    println!("create {:?}", options); // create OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: false, create: true, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Задает параметр для создания гарантированно нового файла.
    let mut options = OpenOptions::new();
    options.create_new(true);
    println!("create_new {:?}", options); // create_new OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: false, create: false, create_new: true, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

    // Открывает файл с заданными параметрами.
    let mut options = OpenOptions::new();
    options.read(true);
    let f = options.open("main.rs");
    println!("open {:?}", f); // open Ok(File { handle: 0xa0, path: "\\\\?\\C:\\Projects\\Rust\\rust-template\\open_options\\src\\main.rs" })

    // Задает параметр для очистки предыдущего файла.
    let mut options = OpenOptions::new();
    options.truncate(true);
    println!("truncate {:?}", options); // truncate OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: true, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })

}
