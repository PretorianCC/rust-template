use std::{fs::File, io::Write};

fn main() {
    
    // Открывает файл на чтение.
    let f = File::open("main.rs").unwrap();
    println!("open {:?}", f); // open File { handle: 0xb0, path: "\\\\?\\C:\\Projects\\Rust\\rust-template\\file\\src\\main.rs" }

    // Открывает файл для записи.
    let f = File::create("1.tmp").unwrap();
    println!("create {:?}", f); // create File { handle: 0x9c, path: "\\\\?\\C:\\Projects\\Rust\\rust-template\\file\\src\\1.tmp" }

    // Возвращает новый объект OpenOptions. Можно использовать для открытия или создания файла с определенными опциями, если open () или create () не подходят.
    let mut opt = File::options();
    println!("options {:?}", opt); // options OpenOptions(OpenOptions { read: false, write: false, append: false, truncate: false, create: false, create_new: false, custom_flags: 0, access_mode: None, attributes: 0, share_mode: 7, security_qos_flags: 0, security_attributes: 0x0 })
    let _ = opt.append(true).open("1.tmp");

    // Метаданные файла.
    let f = File::open("main.rs").unwrap();
    let metadata = f.metadata();
    println!("metadata {:?}", metadata); // metadata Ok(Metadata {...}) ...

    // Усекает или расширяет файл, обновляя его размер. Все промежуточные данные будут заполнены 0.
    let f = File::create("1.tmp").unwrap();
    let len = f.set_len(10);
    println!("set_len {:?}", len); // set_len Ok(())

    // Устанавливает разрешения файлу.
    let f = File::open("1.tmp").unwrap();
    let mut perms = f.metadata().unwrap().permissions();
    perms.set_readonly(false);
    let result = f.set_permissions(perms);
    println!("set_permissions {:?}", result); // set_permissions Err(Os { code: 5, kind: PermissionDenied, message: "Отказано в доступе." })

    // Пытается синхронизировать все внутренние метаданные ОС с диском.
    let mut f = File::create("1.tmp").unwrap();
    f.write_all(b"Hello, world!").unwrap();
    let result = f.sync_all();
    println!("sync_all {:?}", result); // sync_all Ok(())

    // Пытается синхронизировать все внутренние метаданные ОС с диском. Может не синхронизировать метаданные файла с файловой системой.
    let mut f = File::create("1.tmp").unwrap();
    f.write_all(b"Hello, world!").unwrap();
    let result = f.sync_data();
    println!("sync_data {:?}", result); // sync_data Ok(())

    // Создает новый экземпляр File, который использует тот же самый базовый дескриптор файла, что и существующий экземпляр File.
    let f = File::create("1.tmp").unwrap();
    let file_copy = f.try_clone();
    println!("try_clone {:?}", file_copy); // try_clone Ok(File { handle: 0xbc, path: "\\\\?\\C:\\Projects\\Rust\\rust-template\\file\\src\\1.tmp" })

}
