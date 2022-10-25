use std::collections::LinkedList;
use std::iter::FromIterator;

fn main() {

    // Перемещение одного списка в конец другого списка.
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);
    println!("append {:?}", list1);

    // Возвращает неизменяемую ссылку на последний элемент списка или None.
    let mut list = LinkedList::new();
    list.push_back(1);
    println!("back {:?}", list.back());

    
    // Возвращает изменяемую ссылку на последний элемент списка или None.
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    let value = list.back_mut().unwrap();
    *value += 1;
    println!("back_mut {:?}", list);

    // Очищает список, удаляя все элементы.
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.clear();
    println!("clear {:?}", list);

    // Присутствует элемент в списке?
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    println!("contains {}", list.contains(&1));

    // Возвращает неизменяемую ссылку на первый элемент списка или None.
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    println!("front {:?}", list.front());

    // Возвращает изменяемую ссылку на первый элемент списка или None.
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    let value = list.front_mut().unwrap();
    *value += 1;
    println!("front_mut {:?}", list);

    // Список не содержит элементов?
    let list: LinkedList<i32> = LinkedList::new();
    println!("is_empty {:?}", list.is_empty());
    
    // Итератор списка неизменяемых элементов.
    let list = LinkedList::from([1, 2, 3]);
    print!("iter ");
    for item in list.iter() {
        print!("{item} ");
    }
    print!("\n");

    // Итератор списка изменяемых элементов.
    let mut list = LinkedList::from([1, 2, 3]);
    for item in list.iter_mut() {
        *item += 1;
    }
    println!("iter_mut {:?}", list);

    // Возвращает длину списка.
    let list = LinkedList::from([1, 2, 3]);
    println!("len {:?}", list.len());

    // Возвращает новый пустой список.
    let list: LinkedList<i32> = LinkedList::new();
    println!("LinkedList::new {:?}", list);

    // Удаляет и возвращает последний элемент списка.
    let mut list = LinkedList::from([1, 2, 3]);
    println!("pop_back {:?}", list.pop_back());

    // Удаляет и возвращает первый элемент списка.
    let mut list = LinkedList::from([1, 2, 3]);
    println!("pop_front {:?}", list.pop_front());

    // Добавляет элемент в конец списка.
    let mut list = LinkedList::from([1, 2, 3]);
    list.push_back(4);
    println!("push_back {:?}", list);

    // Добавляет элемент в начало списка.
    let mut list = LinkedList::from([1, 2, 3]);
    list.push_front(4);
    println!("push_front {:?}", list);

    // Разделяет список, перемещая элементы в новый список с указанного индекса.
    let mut list = LinkedList::from([1, 2, 3, 4, 5]);
    let new_list = list.split_off(2);
    println!("split_off {:?}, {:?}", list, new_list);
    
    // Возвращает копию списка.
    let a = LinkedList::from([1, 2, 3]);
    let b = a.clone();
    println!("clone {:?}", b);

    // Возвращает новый список.
    let list: LinkedList<i32> = LinkedList::default();
    println!("LinkedList::default {:?}", list);

    // Сравнивает на равенство списки.
    let a = LinkedList::from([1, 2, 3]);
    let b = a.clone();
    println!("oprerator == {:?}", &a == &b);

    // Сравнивает на не равенство списки.
    let a = LinkedList::from([1, 2, 3]);
    let b = a.clone();
    println!("oprerator != {:?}", &a != &b);
    
    // Копирует все значения списка в текущий список.
    let mut a: LinkedList<_> = LinkedList::new();
    let b = LinkedList::from([1, 2, 3]);
    a.clone_from(&b);
    println!("clone_from {:?}", a);

    // Добавляет в список итерируемые элементы другого списока.
    let mut a = LinkedList::from([1, 2, 3]);
    let b = LinkedList::from([1, 2, 3]);
    a.extend(b.iter());
    println!("extend {:?}", a);

    // Создает новый список из итерируемой коллекции.
    let list = LinkedList::from([1, 2, 3]);
    println!("LinkedList::from {:?}", list);

    // Создает новый список из значений из итератора.
    let list = LinkedList::from_iter([1, 2, 3].iter());
    println!("LinkedList::from_iter {:?}", list);

    // Сравнить со списком другой список на больше, меньше и равенство.
    let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
    let b = LinkedList::from([1, 2, 3]);
    let result = a.cmp(&b);
    println!("cmp {:?}", result); // a > b

    // Сравнивает 2 списка и возвращает максимальный.
    let a: LinkedList<i32> = LinkedList::from([1, 2, 3]);
    let b = LinkedList::from([3, 2, 1]);
    let result = a.max(b);
    println!("max {:?}", result);

    // Сравнивает 2 списка и возвращает минимальный.
    let a: LinkedList<i32> = LinkedList::from([1, 2, 3]);
    let b = LinkedList::from([3, 2, 1]);
    let result = a.min(b);
    println!("min {:?}", result);

    // Возвращает текущий список если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. (list.max(a).min(b))
    let a: LinkedList<i32> = LinkedList::from([1, 2, 3]);
    let b = LinkedList::from([4, 5, 6]);
    let list = LinkedList::from([3, 4, 5]);
    let result = list.clamp(a, b);
    println!("min {:?}", result);

    // Сравнить со списком другой список на больше, меньше и равенство.
    let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
    let b = LinkedList::from([1, 2, 3]);
    let result = a.partial_cmp(&b);
    println!("partial_cmp {:?}", result); // Some(a > b)
  
   // Сравнить списки на меньше. (a < b)
   let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
   let b = LinkedList::from([1, 2, 3]);
   let result = a < b;
   println!("operator < {:?}", result);

   // Сравнить списки на меньше или равно. (a <= b)
   let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
   let b = LinkedList::from([1, 2, 3]);
   let result = a <= b;
   println!("operator <= {:?}", result);

   // Сравнить списки на больше. (a > b)
   let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
   let b = LinkedList::from([1, 2, 3]);
   let result = a > b;
   println!("operator > {:?}", result);

   // Сравнить списки на больше или равно. (a >= b)
   let a: LinkedList<i32> = LinkedList::from([3, 2, 1]);
   let b = LinkedList::from([1, 2, 3]);
   let result = a >= b;
   println!("operator >= {:?}", result);

}
