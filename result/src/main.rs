fn main() {
    
    // Значения перечисления.
    let result: Result<i32, &str> = Result::Ok(2);
    println!("Result {:?}", result); // Result Ok(2)
    let result: Result<i32, &str> = Result::Err("Ошибка");
    println!("Result {:?}", result); // Result Err("Ошибка")

    // Напоминает логическое И, возвращает Ok(value) если self и параметр являются Ok, иначе возвращает Err.
    let a: Result<i32, &str> = Result::Ok(7);
    let b: Result<i32, &str> = Result::Err("Ошибка");
    println!("and {:?}", a.and(b)); // and Err("Ошибка")

    // Напоминает логическое ИЛИ, возвращает Ok(value) если self или параметр являются Ok, иначе возвращает Err.
    let a: Result<i32, &str> = Result::Ok(7);
    let b: Result<i32, &str> = Result::Err("Ошибка");
    println!("or {:?}", a.or(b)); // or Ok(7)

    // Напоминает логическое И, возвращает Ok(value) если self и возращаемое значение из функции параметра являются Ok, иначе возвращает Err.
    // Параметр функции это self.
    fn fn1(x: i32) -> Result<i32, &'static str> { Ok(x + 1) }
    let a: Result<i32, &str> = Result::Ok(7);
    println!("and_then {:?}", a.and_then(fn1)); // and_then Ok(8)

    // Напоминает логическое ИЛИ, возвращает Ok(value) если self или возращаемое значение из функции параметра являются Ok, иначе возвращает Err.
    // Параметр функции это self.
    fn fn2(x: i32) -> Result<i32, i32> {
        Ok(x + 2)
    }
    let a: Result<i32, i32> = Err(1);
    println!("or_else {:?}", a.or_else(fn2)); // or_else Ok(3)

    // Клонирует значение перечисления.
    let value = 1;
    let a: Result<&i32, i32> = Ok(&value);
    let b = a.clone();
    println!("clone {:?}", b); // clone Ok(1)

    // Копирует значение перечисления.
    let value = 1;
    let a: Result<&i32, i32> = Ok(&value);
    let b = a.copied();
    println!("copied {:?}", b); // copied Ok(1)

    // Преобразует в перечисление Option.
    let a: Result<u32, &str> = Ok(1);
    let b: Result<u32, &str> = Err("Ошибка");
    println!("ok {:?} {:?}", a.ok(), b.ok()); // ok Some(1) None
    
    // Возращает истину если Result - Ok
    let a: Result<u32, &str> = Ok(1);
    println!("Ok {}", a.is_ok()); // Ok true

    // Возращает истину если Result - Err
    let a: Result<u32, &str> = Err("Ошибка");
    println!("Err {}", a.is_err()); // Err true

    // Итератор дает одно значение из Ok иначе ничего
    let a: Result<u32, &str> = Ok(2);
    for i in a.iter() {
        println!("iter {:?}", i); // iter 2
    }

    // Итератор дает одно значение из Ok иначе ничего
    let mut a: Result<u32, &str> = Ok(2);
    match a.iter_mut().next() {
        Some(v) => *v = 40,
        None => {},
    };
    println!("iter_mut {:?}", a); // iter_mut Ok(40)

    // Копирует значения Result через ссылку, где значения в Result ссылки.
    let a: Result<String, u32> = Ok("Результат".to_string());
    let b: Result<&str, &u32> = a.as_deref();
    println!("as_deref {:?}", b); // as_deref Ok("Результат")

    // Копирует изменяемое значения Result через ссылку, где значения в Result изменяемые ссылки.
    let mut a: Result<String, u32> = Ok("Результат".to_string());
    let b: Result<&mut str, &mut u32> = a.as_deref_mut();
    println!("as_deref_mut {:?}", b) // as_deref_mut Ok("Результат")

}
