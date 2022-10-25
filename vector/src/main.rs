fn main() {

    // Создать вектор
    let list = vec![1, 2, 3, 4, 5];
    println!("Новый вектор {:?}", list);

    // Добавить значение
    let mut list = vec![1, 2, 3, 4, 5];
    list.push(6);
    println!("Добавлен элемент в вектор {:?}", list);

    // Добавить элемент по индексу
    let mut list = vec![1, 2, 3, 4, 5];
    list.insert(2, 7);
    println!("Вставлен новый элемент {:?}", list);

    // Доступ к элементам по индексу
    let list = vec![1, 2, 3, 4, 5];
    println!("Элемент по индексу №1 {}", &list[1]);


    let temp_get = list.get(1);
    let temp_el = match temp_get {
        Some(el) => el,
        None => &0
    };
    println!("Элемент по индексу №2 {}", temp_el);

    // Доступ к элементам по индексу
    let list = vec![1, 2, 3, 4, 5];
    println!("Элемент по индексу №3 {}", list.get(2).unwrap());

    // Получить первый элемент
    let list = vec![1, 2, 3, 4, 5];
    println!("Первый элемент {}", &list.first().unwrap());

    // Получить последний элемент
    let list = vec![1, 2, 3, 4, 5];
    println!("Последний элемент {}", &list.last().unwrap());

    // Количество элементов
    let list = vec![1, 2, 3, 4, 5];
    println!("Количество элементов {}", list.len());

    // Проверка на пустой вектор
    let list = vec![1, 2, 3, 4, 5];
    println!("Пустой вектор? {}", list.is_empty());

    // Ёмкость вектора
    let list = vec![1, 2, 3, 4, 5];
    println!("Ёмкость вектора {}", list.capacity());

    // Создать вектор с заданной ёмкостью
    let vec: Vec<i32> = Vec::with_capacity(11);
    println!("Выделенная ёмкость вектора {}", vec.capacity());

    // Удаляет элемент по индексу возвращая элемент
    let mut list = vec![1, 2, 3, 4, 5];
    list.remove(1);
    println!("Вектор после удаления по индексу {:?}", list);

    // Удалить последний элемент вернув его
    let mut list = vec![1, 2, 3, 4, 5];
    let value = list.pop();
    println!("Забрали последний элемент {}", value.unwrap());

    // Удалить все элементы с вектора
    let mut list = vec![1, 2, 3, 4, 5];
    list.clear();
    println!("Удалены все элементы {:?}", list);

    // Урезать вектор до необходимой длины
    let mut list = vec![1, 2, 3, 4, 5];
    list.truncate(3);
    println!("Урезанный вектор {:?}", list);

    // Передать значения вектора другому вектору добавлением
    let mut list = vec![1, 2, 3, 4, 5];
    let mut list_old = vec![6, 7, 8, 9, 10];
    list.append(&mut list_old);
    println!("Вектор list {:?}", list);
    println!("Вектор list_old {:?}", list_old);
}
