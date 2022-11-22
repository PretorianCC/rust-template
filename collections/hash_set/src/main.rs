use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;

fn main() {
    // Новая коллекция хеш набора.
    let set: HashSet<String> = HashSet::new();
    println!("Новая коллекция {:?}", &set);

    // Добавить значение в набор если его нет.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    println!("Добавить в коллекцию {:?}", &set);

    // Итератор ссылок в коллекции.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    print!("Итератор ");
    for x in set.iter() {
        print!("{x} ");
    }
    print!("\n");

    // Заменить значение в коллекции или добавить в нее.
    let mut set: HashSet<String> = HashSet::new();
    let set_replace = set.replace("ghi".to_string());
    println!("Заменить значение в коллекции или добавить в нее {:?}", set_replace);

    // Удалить значение из коллекции.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    set.remove(&"def".to_string());
    println!("Удалить из коллекции {:?}", &set);

    // Удалить значение из коллекции и вернуть его.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    let return_value = set.take(&"abc".to_string());
    println!("Удалить значение из коллекции и вернуть его {:?}", return_value);


    // Получить ссылку на значение в коллекции.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    let refere = set.get(&"ghi".to_string());
    println!("Получить ссылку на значение {:?}", refere);

    // Размер коллекции.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    let lens = set.len();
    println!("Размер коллекции {}", lens);

    // Очистить коллекцию.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.clear();
    println!("Очистить коллекцию {:?}", set);

    // Коллекция пуста?
    let set: HashSet<String> = HashSet::new();
    println!("Коллекция пуста? {}", set.is_empty());

    // Ёмкость коллекции.
    let set: HashSet<String> = HashSet::new();
    println!("Ёмкость коллекции {}", set.capacity());

    // Новая коллекция хеш набора с минимальной ёмкостью.
    let set: HashSet<i32> = HashSet::with_capacity(4);
    println!("Новая коллекция с минимальной ёмкостью {}", set.capacity());

    // Итератор ссылок в коллекции, удаляет ссылки из коллекции.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    print!("Итератор ");
    for x in set.drain() {
        print!("{x} ");
    }
    print!("\n");
    println!("Итератор ссылок в коллекции, удаляет ссылки из коллекции {:?}", set);

    // Сохраняет элементы в коллекции если функция возвращает true.
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.retain(|&value| value >= 2);
    println!("Сохраняет элементы указанные в функции {:?}", set);

    // Создает случайным образом новый пустой хэш-набор устойчивый к атакам, который будет использоваться для хэш-ключей.
    let s = RandomState::new();
    let mut set: HashSet<i32> = HashSet::with_hasher(s);
    set.insert(2);
    println!("Для хэш-ключей {:?}", set);

    // Создает случайным образом новый пустой хэш-набор устойчивый к атакам минимальной ёмкостью, который будет использоваться для хэш-ключей.
    let s = RandomState::new();
    let mut set: HashSet<i32> = HashSet::with_capacity_and_hasher(10, s);
    set.insert(2);
    println!("Для хэш-ключей с минимальной ёмкостью {:?}", set);

    // Возвращает ссылку на объект BuildHasher.
    let s = RandomState::new();
    let set: HashSet<i32> = HashSet::with_hasher(s);
    let hasher: &RandomState = set.hasher();
    println!("Ссылка на объект BuildHasher {:?}", hasher);

    // Увеличивает ёмкость коллекции. Ни чего не делает если емкость достаточна.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    set.reserve(10);
    println!("Увеличенная ёмкость коллекции {}", set.capacity());

    // Увеличивает ёмкость коллекции, возвращает результат. Ни чего не делает если емкость достаточна.
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    let result = set.try_reserve(10);
    println!("Смогли увеличить ёмкость коллекции? {:?}", result);

    // Уменьшает ёмкость коллекции на сколько возможно (после удаления элементов)
    let mut set = HashSet::with_capacity(100);
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    set.shrink_to_fit();
    println!("Ёмкость коллекции уменьшена {}", set.capacity());

    // Уменьшает ёмкость коллекции на указанный предел, если предел меньше, то операция отменяется
    let mut set = HashSet::with_capacity(100);
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    set.shrink_to(10);
    println!("Ёмкость коллекции уменьшена до значения {}", set.capacity());

    // true если коллекция содержит значение
    let mut set: HashSet<String> = HashSet::new();
    set.insert("abc".to_string());
    println!("В коллекции есть значения? {}", set.contains(&"abc".to_string()));

    // Итератор тех ссылок коллекции которых нет в коллекции параметра
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);
    for x in a.difference(&b) {
        println!("Вычитание одной коллекции из другой {x}");
    }

    // Итератор тех ссылок коллекции которые присутствуют в обеих коллекциях.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);    
    print!("Логическое И коллекций ");
    for x in a.intersection(&b) {
        print!("{x} ");
    }
    print!("\n");
    
    // Коллекции не имеют общих значений?
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 5, 6]);
    println!("Коллекции не имеют общих значений? {}", a.is_disjoint(&b));

    // Коллекция является подмножеством коллекции в параметре?
    let a = HashSet::from([3, 2]);
    let b = HashSet::from([1, 2, 3, 4]);
    println!("Коллекция содержит все значения коллекции в параметре? {}", a.is_subset(&b));
    
    // Коллекция в параметрах содержит все значения текущей коллеции?
    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 2]);
    println!("Коллекция в параметрах содержит все значения текущей коллеции? {}", a.is_superset(&b));

    // Итератор неповторяющихся значений в обеих коллекциях.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([4, 2, 3, 4]);
    print!("Итератор неповторяющихся значений в обеих коллекциях ");    
    for x in a.union(&b) {
        print!("{x} ");
    }
    print!("\n");

    // Возвращает новую коллекцию из пересекаемых элементов в коллекциях.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    let set = &a & &b;
    println!("Коллекция из пересекаемых элементов в коллекциях {:?}", set);

    // Возвращает новую коллекцию из объединенных элементов в коллекциях.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    let set = &a | &b;
    println!("Коллекция из объединенных элементов в коллекциях {:?}", set);

    // Возвращает новую коллекцию по симметричному различию.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    let set = &a ^ &b;
    println!("Коллекция из симетричных элементов в коллекциях {:?}", set);

    // Возвращает копию коллекции.
    let mut set = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    let set_old = set.clone();
    println!("Копия коллекция {:?}", set_old);

    // Копирует все значения коллекции в текущую коллекцию.
    let mut set = HashSet::new();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    let mut set_old = HashSet::new();
    set_old.clone_from(&set);
    println!("Копирует все значения {:?}", set_old);

    // Создает новую коллекцию со значением по умолчанию для хешера.
    let mut set: HashSet<String> = HashSet::default();
    set.insert("abc".to_string());
    set.insert("def".to_string());
    set.insert("ghi".to_string());
    println!("Создает новую коллекцию {:?}", set);


    // Добавляет в коллекцию итерируемое значение.
    let mut set = HashSet::from([1, 2, 3]);
    set.extend([4, 5, 6]);
    println!("Добавляет в коллекцию итерируемое значение {:?}", set);

    // Добавляет в коллекцию значения из итератора.
    let mut set = HashSet::from([1, 2, 3]);
    set.extend([4, 5, 6].iter());
    println!("Добавляет в коллекцию значения из итератора {:?}", set);

    // Создает новую коллекцию из итерируемого значения.
    let set = HashSet::from([1, 2, 3, 4]);
    println!("Создает новую коллекцию {:?}", set);

    // Создает новую коллекцию из значений из итератора.
    let set: HashSet<_> = HashSet::from_iter([1, 2, 3, 4].iter());
    println!("Создает новую коллекцию из значений из итератора {:?}", set);

    // Создает псевдоним типа HashSet<T>.
    type SetMy = HashSet<i32>;
    let mut set: SetMy = HashSet::new();
    set.insert(1);
    println!("Создает новый тип аналогичный HashSet<T> {:?}", set);

    // Возвращает итератор.
    let set = HashSet::from([1, 2, 3]);
    print!("Возвращает итератор для коллекции {{");
    for x in set.into_iter() {
        print!(" {x}");
    }
    println!(" }}");

    // Проверяет на равенство.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([1, 2, 3]);
    println!("Проверяет на равенство {}", &a == &b);

    // Проверяет на не равенство.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([1, 2, 3]);
    println!("Проверяет на не равенство {}", &a != &b);

    // Вычитает из одной коллекции другую и создает новую коллекцию.
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([3, 4, 5]);
    let set = &a - &b;
    println!("Вычитает из одной коллекции другую {:?}", set);

    // Возвращает итератор содержащий элементы присутствующие только в одной из коллекций (xor).
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([3, 4, 5]);
    print!("symmetric_difference ");
    for item in a.symmetric_difference(&b) {
        print!("{} ", item);
    }
    print!("\n");

    // Возвращает итератор содержащий все элементы присутствующие в коллекциях (or).
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([3, 4, 5]);
    print!("union ");
    for item in a.union(&b) {
        print!("{} ", item);
    }
    print!("\n");

}
