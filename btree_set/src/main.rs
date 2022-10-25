use std::collections::BTreeSet;
use std::iter::FromIterator;

fn main() {

    // Добавить элементы из другого списка.
    let mut a = BTreeSet::from([1, 2, 3]);
    let mut b = BTreeSet::from([4, 5, 6]);
    a.append(&mut b);
    println!("append {:?}", a);

    // Очистить список.
    let mut list = BTreeSet::from([1, 2, 3]);
    list.clear();
    println!("clear {:?}", list);

    // Возвращает значение true, если список содержит значение.
    let list = BTreeSet::from([1, 2, 3]);
    println!("contains {:?}", list.contains(&1));

    // Итератор тех ссылок списка которых нет в коллекции параметра.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([3, 4, 5]);
    print!("difference ");
    for item in a.difference(&b) {
        print!("{item} ");
    }
    print!("\n");

    // Возвращает ссылку из списка если есть равное значению поиска.
    let list = BTreeSet::from([1, 2, 3]);
    println!("get {:?}", list.get(&2));

    // Добавление значения в список если его нет.
    let mut list = BTreeSet::from([1, 2, 3]);
    list.insert(4);
    println!("insert {:?}", list);

    // Итератор возвращает пересечения списка в порядке возрастания (AND).
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([3, 2, 5]);
    print!("intersection ");
    for item in a.intersection(&b) {
        print!("{item} ");
    }
    print!("\n");

    // Возвращает true если списки не пересекаются элементами.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([4, 5, 6]);
    let result = a.is_disjoint(&b);
    println!("is_disjoint {}", result);

    // Пустой список?
    let list: BTreeSet<i32> = BTreeSet::new();
    println!("is_empty {}", list.is_empty());

    // Является ли текущий список подмножеством другого списка, т.е. содержит все его элементы.
    let a = BTreeSet::from([2, 3]);
    let b = BTreeSet::from([1, 2, 3, 4]);
    let result = a.is_subset(&b);
    println!("is_subset {}", result);
    
    // Является ли текущий список подмножеством другого списка, т.е. содержит все его элементы. Аналог is_subset.
    let a = BTreeSet::from([2, 3]);
    let b = BTreeSet::from([1, 2, 3, 4]);
    let result = a.is_superset(&b);
    println!("is_superset {}", result);

    // Итератор ссылок в списке.
    let list = BTreeSet::from([1, 2, 3]);
    print!("iter ");
    for item in list.iter() {
        print!("{} ", item);
    }
    print!("\n");

    // Возвращает количество элементов в списке.
    let list = BTreeSet::from([1, 2, 3]);
    println!("len {}", list.len());

    // Возвращает новый пустой список
    let list: BTreeSet<i32> = BTreeSet::new();
    println!("BTreeSet::new {:?}", list);

    // Возвращает итератор элементов вошедших в диапазон по значениям.
    let list = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    print!("range ");
    for item in list.range(2..7) {
        print!("{} ", item);
    }
    print!("\n");
    
    // Удаляет указанный элемент из списка.
    let mut list = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    list.remove(&3);
    println!("remove {:?}", list);

    // Заменяет значение в списке или добавляет в него.
    let mut list: BTreeSet<i32> = BTreeSet::new();
    list.replace(1);
    println!("replace {:?}", list);

    // Сохраняет элементы в коллекции если функция возвращает true.
    let mut list = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    list.retain(|&k| k % 2 == 0);
    println!("retain {:?}", list);

    // Разделяет список, перемещая элементы в новый список с указанного индекса.
    let mut list = BTreeSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let new_list = list.split_off(&4);
    println!("split_off {:?}, {:?}", list, new_list); // split_off {1, 2, 3}, {4, 5, 6, 7, 8, 9}

    // Возвращает итератор содержащий элементы присутствующие только в одной из коллекций (xor).
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    print!("symmetric_difference ");
    for item in a.symmetric_difference(&b) {
        print!("{} ", item);
    }
    print!("\n"); // symmetric_difference 1 2 5 6 

    // Удалить указанное значение из набора и вернуть его если оно есть.
    let mut list = BTreeSet::from([1, 2, 3]);
    let return_value = list.take(&2);
    println!("Удалить значение из списка {:?} {:?}", return_value, list); // Удалить значение из списка Some(2) {1, 3}

    // Возвращает итератор содержащий все элементы присутствующие в коллекциях (or).
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    print!("union ");
    for item in a.union(&b) {
        print!("{} ", item);
    }
    print!("\n"); // union 1 2 3 4 5 6

    // Возвращает новый список из пересекаемых элементов списков, по варианту and.
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    println!("& {:?}", &a & &b); // & {3, 4}

    // Возвращает новый список из объединенных элементов списков, по варианту or.
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    println!("| {:?}", &a | &b); // | {1, 2, 3, 4, 5, 6}

    // Возвращает новый список по симметричному различию, по варианту xor.
    let a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    println!("^ {:?}", &a ^ &b); // ^ {1, 2, 5, 6}

    // Возвращает копию списка.
    let list = BTreeSet::from([1, 2, 3]);
    let list_new = list.clone();
    println!("clone {:?}", list_new); // clone {1, 2, 3}

    // Копирует все значения списка в текущий список.
    let list = BTreeSet::from([1, 2, 3]);
    let mut list_new: BTreeSet<i32> = BTreeSet::new();
    list_new.clone_from(&list);
    println!("clone_from {:?}", list_new); // clone_from {1, 2, 3}

    // Создает новый пустой список.
    let mut list = BTreeSet::default();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    println!("BTreeSet::default {:?}", list); // BTreeSet::default {1, 2, 3}

    // Добавляет в список элементы коллекции.
    let mut a = BTreeSet::from([1, 2, 3, 4]);
    let b = BTreeSet::from([3, 4, 5, 6]);
    a.extend(&b);
    println!("extend {:?}", a); // extend {1, 2, 3, 4, 5, 6}

    // Новый список из массива.
    let list = BTreeSet::from([1, 2, 3]);
    println!("BTreeSet::from {:?}", list); // BTreeSet::from {1, 2, 3}

    // Создает новый список из итератора коллекции.
    let list = BTreeSet::from_iter([1, 2, 3].iter());
    println!("BTreeSet::from_iter {:?}", list); // BTreeSet::from_iter {1, 2, 3}

    // Возвращает итератор для перемещения содержимого.
    let list = BTreeSet::from([1, 2, 3]);
    print!("into_iter {{");
    for x in list.into_iter() {
        print!(" {x}");
    }
    println!(" }}"); // into_iter { 1 2 3 }

    // Сравнить со списком другой список на больше, меньше или равенство.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([4, 5, 6]);
    let result = a.cmp(&b);
    println!("cmp {:?}", result); // cmp Less

    // Сравнивает 2 списка и возвращает максимальный.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([4, 5, 6]);
    let result = a.max(b);
    println!("max {:?}", result); // max {4, 5, 6}

    // Сравнивает 2 списка и возвращает минимальный.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([4, 5, 6]);
    let result = a.min(b);
    println!("min {:?}", result); // min {1, 2, 3}

    // Возвращает текущий список если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. (list.max(value\_min).min(value\_max))
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([4, 5, 6]);
    let list = BTreeSet::from([3, 4, 5]);
    let result = list.clamp(a, b);
    println!("clamp {:?}", result); // clamp {3, 4, 5}

    // Проверить на равенство списки.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator == {:?}", &a == &b); // oprerator == true

    // Проверить на не равенство списки.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator != {:?}", &a != &b); // oprerator != false
   
    // Сравнить со списком другой список на больше, меньше или равенство.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    let result = a.partial_cmp(&b);
    println!("partial_cmp {:?}", result); // partial_cmp Some(Equal)

    // Сравнить списки на меньше. (a < b)
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator < {:?}", &a < &b); // oprerator < false

    // Сравнить списки на меньше или равен. (a <= b)
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator <= {:?}", &a <= &b); // oprerator <= true

    // Сравнить списки на больше. (a > b)
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator > {:?}", &a > &b); // oprerator > false

    // Сравнить списки на больше или равно. (a >= b)
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([1, 2, 3]);
    println!("oprerator >= {:?}", &a >= &b); // oprerator >= true

    // Вычитает из одного списка другой и создает новый список.
    let a = BTreeSet::from([1, 2, 3]);
    let b = BTreeSet::from([3, 4, 5]);
    println!("oprerator - {:?}", &a - &b); // oprerator - {1, 2}


}
