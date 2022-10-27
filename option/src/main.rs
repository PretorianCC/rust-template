use std::pin::Pin;

fn main() {
    
    // Значения перечисления.
    let result: Option<u32> = Some(2);
    println!("Option {:?}", result); // Option Some(2)
    let result: Option<u32> = None;
    println!("Option {:?}", result); // Option None

    // Значение Some?
    let result: Option<u32> = Some(2);
    println!("is_some {}", result.is_some()); // is_some true

    // Значение None?
    let result: Option<u32> = Some(2);
    println!("is_none {}", result.is_none()); // is_none false

    // Напоминает логическое И, возвращает Some(value) параметра если self и параметр являются Some, иначе возвращает None.
    let a: Option<u32> = Some(2);
    let b: Option<u32> = Some(3);
    let c: Option<u32> = None;
    println!("and {:?}", a.and(b)); // and Some(3)
    println!("and {:?}", c.and(b)); // and None
    
    // Напоминает логическое И, возвращает Some(value) если self и возращаемое значение из функции параметра являются Some, иначе возвращает None. Параметр функции это self.
    fn fn1(x: u32) -> Option<u32> { Some(x + 1) }
    let a: Option<u32> = Some(2);
    println!("and_then {:?}", a.and_then(fn1)); // and_then Some(3)

    // Преобразует Option<T> (или &Option<T>) в результат Option<&T::Target>. Копирует вариант Some исходного результата через Deref и возвращает новый результат.
    let a: Option<&str> = Some("Ок");
    let b = a.as_deref();
    println!("as_deref {:?}", b); // as_deref Some("Ок")

    // Преобразует &mut Option<T> в Option<&mut T>
    let mut a: Option<&str> = Some("Ок");
    let b = a.as_mut();
    println!("as_mut {:?}", b); // as_mut Some("Ок")

    // Преобразует Pin<&mut Option<T>> в Option<Pin<&mut T>>
    let mut a: Option<&str> = Some("Ок");
    let b = Pin::new(&mut a);
    let result = b.as_pin_mut();
    println!("as_pin_mut {:?}", result); // as_pin_mut Some("Ок")

    // Преобразует Pin<&Option<T>> в Option<Pin<&T>>
    let a = Pin::new(&Some("Ок"));
    let result = a.as_pin_ref();
    println!("as_pin_ref {:?}", result); // as_pin_ref Some("Ок")

    // Преобразует &Option<T> в Option<&T>
    let a: Option<&str> = Some("Ок");
    let b = &a.as_ref();
    println!("as_ref {:?}", b); // as_ref Some("Ок")

    // Глубокое копирование, копируются все значения ссылок.
    let a = Some(&1);
    let b = a.clone();
    println!("clone {:?}", b); // clone Some(1)

    // Не глубокое копирование, копирование ссылок.
    let a = Some(&1);
    let b = a.copied();
    println!("copied {:?}", b); // copied Some(1)

    // Возвращает содержащееся значение в Some, или паника с сообщением.
    let a = Some(2);
    let b = a.expect("None");
    println!("expect {:?}", b); // expect 2

    // Возвращает None если self None или результат функции не истина.
    fn is_even(n: &i32) -> bool {
        n % 2 == 0
    }
    let a = Some(4);
    println!("filter {:?}", a.filter(is_even)); // filter Some(4)

    // Убирает один вложенный Some.
    let a = Some(Some(2));
    println!("flatten {:?}", a.flatten()); // flatten Some(2)

    // Заменить или вернуть значение если оно None.
    let mut a = None;
    a.get_or_insert(7);
    println!("get_or_insert {:?}", a); // flatten Some(2)

    // Заменить или вернуть значение если оно None вычисленное в функции.
    let mut a = None;
    a.get_or_insert_with(|| 5);
    println!("get_or_insert_with {:?}", a); // get_or_insert_with Some(5)

    // Заменить и вернуть значение.
    let mut a: Option<i32> = Some(4);
    let _ = a.insert(5);
    println!("insert {:?}", a); // insert Some(5)

    // Итератор. 1 значение при Some, пустой при None.
    let a = Some(2);
    for i in a.iter() {
        println!("iter {:?}", i); // iter 2
    }

    // Итератор изменяемого Option. 1 значение при Some, пустой при None.
    let mut a = Some(2);
    for i in a.iter_mut() {
        *i = 5;
        println!("iter {:?}", i); // iter 2
    }

    // Применяет функцию к содержащемуся значению, возвращает результат.
    let a = Some(2);
    let b = a.map(|x| x + 1);
    println!("map {:?}", b); // map Some(3)


}
