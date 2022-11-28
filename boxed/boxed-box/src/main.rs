// Тип указателя для выделения памяти в кучи.

fn main() {
    
    // Выделяет память в куче, а затем помещает в нее значение.
    let value = Box::new(5);
    println!("Box::new {:?}", value); // Box::new 5

    // Получает необработанный указатель на значение в куче.
    let value = Box::new(5);
    let ptr = Box::into_raw(value);
    let x = unsafe {Box::from_raw(ptr)};
    println!("from_raw {:?}", x); // from_raw 5

    // Закрепляет значение в куче и гарантирует его не перемещение.
    let value = Box::new(5);
    let result = Box::into_pin(value);
    println!("into_pin {:?}", result); // into_pin 5

    // Получает указатель на значение в куче.
    let value = Box::new(5);
    let ptr = Box::into_raw(value);
    println!("into_raw {:?}", ptr); // ptr 0x280641054d0

    // Создает не перемещаемый указатель на значение в куче.
    let value = Box::pin(5);
    println!("Box::pin {:?}", value); // Box::pin 5

    // Создает указатель и возвращает ссылку на указатель, в куче значение остается после выхода из функции которая возвращает ссылку на это значение.
    let x = vec![1, 2, 3].into_boxed_slice();
    let static_ref = Box::leak(x);
    static_ref[0] = 4;
    println!("Box::leak {:?}", static_ref); // Box::leak [4, 2, 3]

}
