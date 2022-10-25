use std::{fs::{File, OpenOptions}, io::{Write, Read}};

fn main() {

    let path = "text.txt";
    let text = "Текст для записи".to_string();

    // Создать новый файл или очистить имеющийся.
    let mut f = File::create(path).expect("Ошибка создания файла!");

    // Записать текст в файл.
    f.write_all(text.as_bytes()).expect("Ошибка записи в файл.");

    let mut file_data = String::new();

    // Открыть файл на чтение.
    let mut f = File::open(path).expect("Ошибка открытия файла.");
    f.read_to_string(&mut file_data).expect("Ошибка чтения файла.");
    println!("{}", file_data);

    // Запись-чтение файла.
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("Ошибка открытия/создания файла.");

    let mut file_data = String::new();
    f.read_to_string(&mut file_data).expect("Ошибка чтения файла.");
    println!("{}", file_data);

    let new_path = "data.txt";

    // Переименование файла.
    std::fs::rename(path, new_path).expect("Ошибка переименования файла.");

    // Удаление файла.
    std::fs::remove_file(new_path).expect("Ошибка удаления файла.");
    
}
