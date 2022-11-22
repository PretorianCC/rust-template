use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::ops::Index;

fn main() {

    // Возвращает число элементов, которые коллекция может содержать без выделения памяти.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("capacity {:?}", map.capacity());

    // Очищает коллекцию, удаляя все пары ключ-значение.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    map.clear();
    println!("clear {:?}", map);

    // Проверяет, есть ли значение у ключа?
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("contains_key {}", map.contains_key("a"));

    // Итератор ссылок в коллекции, удаляет ссылки из коллекции.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("drain {{ ");
    for item in map.drain() {
        print!("{:?} ", item);
    }
    println!("}}");

    // Возвращает результат Entry поиска по ключу.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("entry {:?}", map.entry("a"));

    // Возвращает результат поиска неизменяемого значения по ключу.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("get {:?}", map.get("a"));

    // Возвращает результат поиска изменяемого значения по ключу.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("get_mut {:?}", map.get_mut("a"));

    // Возвращает результат поиска ключ-значение по ключу.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("get_key_value {:?}", map.get_key_value("a"));

    
    // Возвращает ссылку на объект BuildHasher.
    let s = RandomState::new();
    let map: HashMap<&str, i32> = HashMap::with_hasher(s);
    let hasher: &RandomState = map.hasher();
    println!("with_hasher {:?}", hasher);

    // Вставить ключ-значение.
    let mut map = HashMap::new();
    map.insert(String::from("first"), 1);
    map.insert(String::from("second"), 2);
    map.insert(String::from("three"), 3);
    map.insert(String::from("four"), 4);
    println!("insert {:?}", map);

    // Увеличивает ёмкость коллекции.
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("abc", 1);
    map.reserve(10);
    println!("reserve {}", map.capacity());

    // Возвращает итератор ключей коллекции. Потребляет коллекцию.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("into_keys ");
    for key in map.into_keys() {
        print!("{} ", key);
    }
    print!("\n");

    // Возвращает итератор значений коллекции. Потребляет коллекцию.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("into_values ");
    for value in map.into_values() {
        print!("{} ", value);
    }
    print!("\n");

    // Проверяет пустая коллекция?
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("is_empty {}", map.is_empty());

    // Итератор не мутабельных ссылок в коллекции.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("iter ");
    for item in map.iter() {
        print!("{:?} ", item);
    }
    print!("\n");

    // Итератор мутабельных ссылок в коллекции.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("iter_mut ");
    for item in map.iter_mut() {
        print!("{:?} ", item);
    }
    print!("\n");

    // Возвращает итератор ключей коллекции.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("keys ");
    for key in map.keys() {
        print!("{} ", key);
    }
    print!("\n");

    // Количество элементов в коллекции коллекции.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("len {}", map.len());

    // Создает новую пустую коллекцию.
    let map: HashMap<&str, i32> = HashMap::new();
    println!("HashMap::new {:?}", map);

    // Удаляет элемент, возвращая удаляемый ключ.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let key = map.remove("a");
    println!("remove {:?} {:?}", key, map);

    // Удаляет элемент, возвращая удаляемый ключ-значение.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let key = map.remove_entry("a");
    println!("remove_entry {:?} {:?}", key, map);

    // Сохраняет в коллекции только элементы удовлетворяют функцию.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    map.retain(|_, &mut value| value % 2 != 0);
    println!("retain {:?}", map);
    
    // Уменьшает ёмкость коллекции на указанный предел, если предел меньше, то операция отменяется.
    let mut map = HashMap::with_capacity(100);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.shrink_to(10);
    println!("shrink_to {}", map.capacity());

    // Уменьшает ёмкость коллекции на сколько возможно (после удаления элементов).
    let mut map = HashMap::with_capacity(100);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.shrink_to_fit();
    println!("shrink_to_fit {}", map.capacity());

    // Увеличивает ёмкость коллекции, возвращает результат. Ни чего не делает если емкость достаточна.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let result = map.try_reserve(10);
    println!("try_reserve {:?}", result);
    
    // Возвращает итератор неизменяемых значений коллекции.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("values ");
    for value in map.values() {
        print!("{} ", value);
    }
    print!("\n");

    // Возвращает итератор изменяемых значений коллекции.
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    print!("values_mut ");
    for value in map.values_mut() {
        *value = *value + 10;
        print!("{} ", value);
    }
    print!("\n");

    // Новая коллекция с минимальной ёмкостью.
    let map: HashMap<&str, i32> = HashMap::with_capacity(4);
    println!("with_capacity {}", map.capacity());

    // Создает случайным образом новую коллекцию устойчивую к атакам минимальной ёмкостью.
    let s = RandomState::new();
    let map: HashMap<&str, i32> = HashMap::with_capacity_and_hasher(10, s);
    println!("with_capacity_and_hasher {:?}", map);

    // Создает случайным образом новую коллекцию устойчивую к атакам.
    let s = RandomState::new();
    let map: HashMap<&str, i32> = HashMap::with_hasher(s);
    println!("with_hasher {:?}", map);

    // Копирует все значения из одной коллекции в новую.
    let a = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let b = a.clone();
    println!("clone {:?}", b);

    // Копирует все значения из коллекции в текущую коллекцию.
    let mut a: HashMap<&str, i32> = HashMap::new(); 
    
    a.clone_from(&b);
    println!("clone_from {:?}", a);

    // Создает новую коллекцию со значением по умолчанию для хешера.
    let mut map: HashMap<&str, i32> = HashMap::default();
    map.insert("a", 1);
    println!("HashMap::default {:?}", map);

    // Добавляет в коллекцию элементы с итерируемого значения.
    let mut map = HashMap::from([("a", 1), ("b", 2),]);
    map.extend([("c", 3)]);
    println!("extend {:?}", map);

    // Новая коллекция из массива.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("HashMap::from {:?}", map);


    // Возвращает элемент по ключу из коллекции.
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("index {:?}", map.index("a"));

    // Проверяет на равенство коллекций.
    let a = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let b = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("Оператор == {:?}", &a == &b);
   
    // Проверяет на не равенство коллекций.
    let a = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    let b = HashMap::from([("a", 1), ("b", 2), ("c", 3),]);
    println!("Оператор == {:?}", &a != &b);
   
}
