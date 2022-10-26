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
    
    // Преобразует в перечисление Option.
    let a: Result<u32, &str> = Ok(1);
    let b: Result<u32, &str> = Err("Ошибка");
    println!("err {:?} {:?}", a.err(), b.err()); // err None Some("Ошибка")
    
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
    println!("as_deref_mut {:?}", b); // as_deref_mut Ok("Результат")

    // Преобразует значение в изменяемую ссылку из изменяемого Result.
    let mut a: Result<String, u32> = Ok("Результат".to_string());
    let result: Result<&mut String, &mut u32> = a.as_mut();
    println!("as_mut {:?}", result); // as_mut Ok("Результат")

    // Преобразует значение в ссылку.
    let a: Result<String, u32> = Ok("Результат".to_string());
    let result: Result<&String, &u32> = a.as_ref();
    println!("as_ref {:?}", result); // as_ref Ok("Результат")
    
    // Возвращает содержащееся значение в OK, или паника с сообщением.
    let a: Result<String, u32> = Ok("Результат".to_string());
    let b = a.expect("Ошибка");
    println!("expect {:?}", b); // expect "Результат"

    // Возвращает содержащееся значение в Err, или паника с сообщением.
    let a: Result<u32, &str> = Err("Результат ошибки");
    let b = a.expect_err("Ошибка");
    println!("expect_err {:?}", b); // expect_err "Результат ошибки"   
    
    // Применяет функцию к содержащемуся значению Ok. Err пропускает.
    let line = "1\n2\n3\n4\n";
    print!("map ");
    for num in line.lines() {
        match num.parse::<i32>().map(|i| i * 2) {
            Ok(n) => print!("{n} "),
            Err(..) => {}
        }
    }
    print!("\n"); // map 2 4 6 8

    // Применяет функцию к содержащемуся значению Err. Ok пропускает.
    let a: Result<i32, i32> = Err(2);
    println!("map_err {:?}", a.map_err(|i| i * 2)); //map_err Err(4)

    // Применяет функцию к значению если Ok иначе значение по умолчанию.
    let a: Result<i32, &str> = Ok(2);
    println!("map_or {:?}", a.map_or(42, |i| i * 2)); // map_or 4

    // Применяет функции к значениям Err или Ok.
    let a: Result<i32, i32> = Err(2);
    println!("map_or_else {:?}", a.map_or_else(|i| i * 3, |i| i * 2)); // map_or_else 6

    // Меняет местами обертки Result и Option. Result(Option) -> Option(Result).
    let a: Result<Option<i32>, &str> = Ok(Some(5));
    println!("transpose {:?}", a.transpose()); // transpose Some(Ok(5))

    //Возвращает содержащееся значение в OK, или паника.
    let a: Result<i32, i32> = Ok(2);
    println!("unwrap {:?}", a.unwrap()); // unwrap 2

    // Возвращает содержащееся значение в Err, или паника.
    let a: Result<i32, &str> = Err("Ошибка");
    println!("unwrap_err_unchecked {:?}", unsafe { a.unwrap_err_unchecked() }); // unwrap_err_unchecked "Ошибка"

    // Возвращает содержащееся значение OK или предоставленное значение по умолчанию.
    let a: Result<i32, &str> = Err("Ошибка");
    println!("unwrap {:?}", a.unwrap_or(3)); // unwrap 3

    // Возвращает содержащееся значение в OK, или значение по умолчанию для данного типа.
    let a: Result<i32, &str> = Err("Ошибка");
    println!("unwrap_or_default {:?}", a.unwrap_or_default()); // unwrap_or_default 0

    // Возвращает содержащееся значение в OK или предоставленное значение по умолчанию работы функции.
    fn count(x: &str) -> usize { x.len() }
    let a = Err("Ошибка");
    println!("unwrap_or_else {}", a.unwrap_or_else(count)); // unwrap_or_else 12

    // Возвращает содержащееся значение OK, без проверки того, что значение не является ошибкой.
    let a: Result<i32, &str> = Ok(2);
    println!("unwrap_unchecked {:?}", unsafe { a.unwrap_unchecked() }); // unwrap_unchecked 2

}
