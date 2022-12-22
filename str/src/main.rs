// Срез строки.

use std::net::ToSocketAddrs;

fn main() {
    
    // Преобразует строковый срез из UTF-8 в байтовый. 
    let bytes = "bors".as_bytes();
    println!("as_bytes {:?}", bytes); // as_bytes [98, 111, 114, 115]

    // Преобразует изменяемый строковый срез из UTF-8 в байтовый.  unsafe
    let mut string = String::from("Hello");
    let bytes = unsafe { string.as_bytes_mut() };
    println!("as_bytes_mut {:?}", bytes); // as_bytes_mut [72, 101, 108, 108, 111]

    // Указатель на изменяемый срез строки.
    let mut string = String::from("Hello");
    let (first, _) = string.split_at_mut(2);
    let ptr = first.as_mut_ptr();
    println!("as_mut_ptr {:?}", ptr); // as_mut_ptr 0x185343754d0

    // Указатель на срез строки.
    let ptr = "first".as_ptr();
    println!("as_ptr {:?}", ptr); // as_ptr 0x7ff7e229f450

    // Итератор по байтам строкового среза.
    let bytes = "bors".bytes();
    print!("bytes ");
    for byte in bytes {
        print!("{byte} ");
    }
    println!(); // bytes 98 111 114 115

    // Возвращает итератор символов и их позиции.
    let str = "help";
    print!("char_indices ");
    for (pos, char) in str.char_indices() {
        print!("{pos}-{char} ");
    }
    println!(); // char_indices 0-h 1-e 2-l 3-p

    // Возвращает итератор символов.
    let str = "help";
    print!("chars ");
    for char in str.chars() {
        print!("{char} ");
    }
    println!(); // chars h e l p

    // true если срез строки содержит шаблон.
    let str = "help";
    println!("contains {:?}", str.contains("el")); // contains true

    // Конвертирует в UTF-16.
    let str = "help";
    println!("encode_utf16 {:?}", str.encode_utf16().count()); // encode_utf16 4

    // true если срез строки заканчивается шаблоном.
    let str = "help";
    println!("ends_with {:?}", str.ends_with("lp")); // ends_with true
    
    // Проверяет, что два среза строки равны без учета регистра.
    let str = "help";
    println!("eq_ignore_ascii_case {:?}", str.eq_ignore_ascii_case("Help")); // eq_ignore_ascii_case true

    // Возвращает итератор символов, графемы, которые начинают строку, будут удалены.
    let str = "help";
    print!("escape_debug ");
    for char in str.escape_debug() {
        print!("{char}");
    }
    println!(); // escape_debug help

    // Возвращает итератор символов.
    let str = "help";
    print!("escape_default ");
    for char in str.escape_default() {
        print!("{char}");
    }
    println!(); // escape_default help

    // Возвращает итератор кодов utf символов.
    let str = "help";
    print!("escape_unicode ");
    for char in str.escape_unicode() {
        print!("{char}");
    }
    println!(); // escape_unicode \u{68}\u{65}\u{6c}\u{70}

    // Возращает индекс найденной подстроки.
    let str = "help";
    println!("find {:?}", str.find("el")); // find Some(1)

    // Возвращает подмножество str.
    let str = "help";
    println!("get {:?}", str.get(1..=2)); // get Some("el")

    // Возвращает изменяемое подмножество str.
    let mut string = String::from("Hello");
    let (first, _) = string.split_at_mut(3);
    println!("get_mut {:?}", first.get_mut(1..=2)); // get_mut Some("el")

    // Возвращает подмножество строки. unsafe
    let str1 = "hello";
    let str2 = unsafe {str1.get_unchecked(..3)};
    println!("get_unchecked {:?}", str2); // get_unchecked "hel"

    // Возвращает изменяемое подмножество str. unsafe
    let mut string = String::from("Hello");
    let (first, _) = string.split_at_mut(3);
    let str = unsafe {
        first.get_unchecked_mut(..3)
    };
    println!("get_unchecked_mut {:?}", str); // get_unchecked_mut "Hel"

    // Находятся ли все символы в этой строке в диапазоне ASCII?
    let str = "hello";
    println!("is_ascii {:?}", str.is_ascii()); // is_ascii true

    // Является байт по указанному индексу первым в символе utf-8?
    let str = "Привет";
    println!("is_char_boundary {:?}", str.is_char_boundary(2)); // is_char_boundary true

    // Пустой срез строки?
    let str = "hello";
    println!("is_empty {:?}", str.is_empty()); // is_empty false

    // Длина среза в байтах.
    let str = "Привет";
    println!("len {:?}", str.len()); // len 12

    // Возвращает итератор по строкам в срезе.
    let str = "hello\nworld";
    print!("lines ");
    for item in str.lines() {
        print!("{item} ");
    }
    println!(); // lines hello world

    // Переводит ascii символы среза в нижний регистр.
    let mut string = String::from("HELLO world!");
    let (first, _) = string.split_at_mut(8);
    first.make_ascii_lowercase();
    println!("make_ascii_lowercase {:?}", first); // make_ascii_lowercase "hello wo"

    // Переводит ascii символы среза в верхний регистр.
    let mut string = String::from("hello world!");
    let (first, _) = string.split_at_mut(8);
    first.make_ascii_uppercase();
    println!("make_ascii_uppercase {:?}", first); // make_ascii_uppercase "HELLO WO"

    // Возращает итератор кортежей с совпадениями по шаблону.
    let str = "abcdeabc";
    print!("match_indices ");
    for item in str.match_indices("abc") {
        print!("{:?} ", item);
    }
    println!(); // match_indices (0, "abc") (5, "abc") 

    // Возращает итератор срезов с совпадениями по шаблону.
    let str = "1ab2cde3abc";
    print!("matches ");
    for item in str.matches(char::is_numeric) {
        print!("{:?} ", item);
    }
    println!(); // matches "1" "2" "3"

    // Парсинг среза строки к нужному типу.
    let str = "4";
    println!("parse {:?}", str.parse::<u32>()); // parse Ok(4)

    // Новая строка повторением среза.
    let str = "abc";
    let string = str.repeat(3);
    println!("repeat {}", string); // repeat abcabcabc

    // Новый срез с заменой всех совпадений по шаблону.
    let str = "abcdeabc";
    let new_str = str.replace("abc", "cba");
    println!("replace {}", new_str); // replace cbadecba

    // Новый срез с заменой первых N совпадений по шаблону.
    let str = "abcdeabc";
    let new_str = str.replacen("abc", "cba", 1);
    println!("replacen {}", new_str); // replacen cbadeabc

    // Возвращает индекс байта для первого символа последнего совпадения шаблона в срезе.
    let str = "abcdeabc";
    println!("rfind {:?}", str.rfind("abc")); // rfind Some(5)

    // Возращает итератор срезов с совпадениями по шаблону в обратном порядке.
    let str = "abcdeabc";
    print!("rmatch_indices ");
    for item in str.rmatch_indices("abc") {
        print!("{:?} ", item);
    }
    println!(); // rmatch_indices (5, "abc") (0, "abc")

    // Возращает итератор срезов с совпадениями по шаблону в обратном порядке.
    let str = "1ab2cde3abc";
    print!("rmatches ");
    for item in str.rmatches(char::is_numeric) {
        print!("{:?} ", item);
    }
    println!(); // rmatches "3" "2" "1"

    // Разделяет строковой срез шаблоном и возращает итератор с обратной последовательностю. Пустые подстроки войдут в итератор.
    let vector: Vec<&str> = "Mary had a little lamb".rsplit(' ').collect();
    println!("rsplit {:?}", vector); // rsplit ["lamb", "little", "a", "had", "Mary"]

    // Разделяет строковой срез в месте последнего совпадения с шаблоном.
    let typle = "cfg=foo=bar".rsplit_once('=');
    println!("rsplit_once {:?}", typle); // rsplit_once Some(("cfg=foo", "bar"))

    // Разделяет строковой срез шаблоном и возращает итератор с обратной последовательностью. Пустые подстроки не войдут в итератор.
    let typle: Vec<&str> = "cfg=foo=bar=".rsplit_terminator('=').collect();
    println!("rsplit_terminator {:?}", typle); // rsplit_terminator ["bar", "foo", "cfg"]

    // Разделяет строковой срез шаблоном и возращает итератор с обратной последовательностю. Пустые подстроки войдут в итератор. Ограничивается количеством разделения.
    let vector: Vec<&str> = "Mary had a little lamb".rsplitn(3, ' ').collect();
    println!("rsplitn {:?}", vector); // rsplitn ["lamb", "little", "Mary had a"]

    // Разделяет строковой срез шаблоном и возращает итератор. Пустые подстроки войдут в итератор.
    let vector: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    println!("split {:?}", vector); // split ["Mary", "had", "a", "little", "lamb"]

    // Разделяет строковой срез по пробельным ascii символам и возращает итератор. Пустые подстроки не войдут в итератор.
    let vector: Vec<&str> = "Mary had a little  lamb".split_ascii_whitespace().collect();
    println!("split_ascii_whitespace {:?}", vector); // split_ascii_whitespace ["Mary", "had", "a", "little", "lamb"]

    // Разделить строковой срез по индексу. Должно быть по границе utf-8 символа.
    let str = "abcdef";
    println!("split_at {:?}", str.split_at(3)); // split_at ("abc", "def")

    // Разделить изменяемый строковой срез по индексу. Должно быть по границе utf-8 символа.
    let mut string = String::from("Mary had a little lamb");
    let (first, _) = string.split_at_mut(17);
    println!("split_at_mut {:?}", first.split_at_mut(8)); // split_at_mut ("Mary had", " a little")

    // Разделяет строковой срез шаблоном и возращает итератор, шаблон так же войдет в результат. Пустые подстроки войдут в итератор.
    let vector: Vec<&str> = "Mary had a little lamb".split_inclusive(' ').collect();
    println!("split_inclusive {:?}", vector); // split_inclusive ["Mary ", "had ", "a ", "little ", "lamb"]

    // Разделяет строковой срез шаблоном и возращает префикс и суфикс.
    let str = "cfg=foo=bar";
    println!("split_once {:?}", str.split_once('=')); // split_once Some(("cfg", "foo=bar"))
    
    // Разделяет строковой срез шаблоном и возращает итератор. Пустые подстроки не войдут в итератор.
    let typle: Vec<&str> = "cfg=foo=bar=".split_terminator('=').collect();
    println!("split_terminator {:?}", typle); // split_terminator ["cfg ", "foo", "bar"]

    // Разделяет строковой срез по пробельным символам и возращает итератор. Пустые подстроки не войдут в итератор.
    let vector: Vec<&str> = "Mary had a little  lamb".split_whitespace().collect();
    println!("split_whitespace {:?}", vector); // split_whitespace ["Mary", "had", "a", "little", "lamb"]

    // Разделяет строковой срез шаблоном и возращает итератор. Пустые подстроки войдут в итератор. Ограничивается количеством разделения.
    let vector: Vec<&str> = "Mary had a little lamb".splitn(3, ' ').collect();
    println!("splitn {:?}", vector); // splitn ["Mary", "had", "a little lamb"]

    // Срез строки начинается с данного шаблона?
    let str = "abcdef";
    println!("starts_with {}", str.starts_with("abc")); // starts_with true

    // Возвращает срез строки с удаленным началом.
    let str = "abcdef";
    println!("strip_prefix {:?}", str.strip_prefix("abc")); // strip_prefix Some("def")

    // Возвращает срез строки с удаленным концом.
    let str = "abcdef";
    println!("strip_suffix {:?}", str.strip_suffix("def")); // strip_suffix Some("abc")

    // Возвращает срез с символами ASCII в нижнем регистре, остальные без изменений.
    let str = "HELLO world!";
    println!("to_ascii_lowercase {:?}", str.to_ascii_lowercase()); // to_ascii_lowercase "hello world!"

    // Возвращает срез с символами ASCII в верхнем регистре, остальные без изменений.
    let str = "HELLO world!";
    println!("to_ascii_uppercase {:?}", str.to_ascii_uppercase()); // to_ascii_uppercase "HELLO WORLD!"

    // Возвращает новую строку в нижнем регистре.
    let str = "Привет МИР!";
    println!("to_lowercase {:?}", str.to_lowercase()); // to_lowercase "привет мир!"

    // Возвращает новую строку в верхнем регистре.
    let str = "Привет МИР!";
    println!("to_uppercase {:?}", str.to_uppercase()); // to_uppercase "ПРИВЕТ МИР!"

    // Возвращает срез строки с удаленными начальными и конечными пробельными символами.
    let str = " abc\n ";
    println!("trim {:?}", str.trim()); // trim "abc"

    // Возвращает срез строки с удаленными конечными пробельными символами.
    let str = " abc\n ";
    println!("trim_end {:?}", str.trim_end()); // trim_end " abc"

    // Возвращает срез строки с удаленным шаблоном в конце.
    let str = "abc123";
    println!("trim_end_matches {:?}", str.trim_end_matches(char::is_numeric)); // trim_end_matches "abc"

    // Возвращает срез строки с удаленным шаблоном в начале и конце.
    let str = "123abc123";
    println!("trim_matches {:?}", str.trim_matches(char::is_numeric)); // trim_matches "abc"

    // Возвращает срез строки с удаленными начальными пробельными символами.
    let str = " \nabc";
    println!("trim_start {:?}", str.trim_start()); // trim_start "abc"

    // Возвращает срез строки с удаленным шаблоном в начале.
    let str = "123abc";
    println!("trim_start_matches {:?}", str.trim_start_matches(char::is_numeric)); // trim_start_matches "abc"

    // Сравнить строки на равенство.
    let str1 = "abc";
    let str2 = "123";
    println!("== {:?}", str1 == str2); // == false

    // Сравнить строки на не равенство.
    let str1 = "abc";
    let str2 = "123";
    println!("!= {:?}", str1 != str2); // != true

    // Сравнить строки на меньше.
    let str1 = "abc";
    let str2 = "123";
    println!("< {:?}", str1 < str2); // < false

    // Сравнить строки на больше.
    let str1 = "abc";
    let str2 = "123";
    println!("> {:?}", str1 > str2); // > true

    //  Сравнить строки на меньше или равно.
    let str1 = "abc";
    let str2 = "123";
    println!("<= {:?}", str1 <= str2); // <= false

    //  Сравнить строки на больше или равно.
    let str1 = "abc";
    let str2 = "123";
    println!(">= {:?}", str1 >= str2); // >= true

    // Получить новый срез по диапазону.
    let str = "abc123";
    println!("[n1..ns] {:?}", &str[1..4]); // [n1..ns] "bc1"

    // Клонирует срез.
    let str = "abc123";
    println!("to_owned {:?}", str.to_owned()); // to_owned "abc123"

    // Клонирует срез в строку.
    let str = "abc123";
    let mut string = String::new();
    str.clone_into(&mut string);
    println!("clone_into {:?}", string); // clone_into "abc123"

    // Срез в сокет адрес.
    let str = "127.0.0.1:80";
    println!("to_socket_addrs {:?}", str.to_socket_addrs()); // to_socket_addrs Ok(IntoIter([127.0.0.1:80]))

    // Преобразовать срез в строку.
    let str = "abc123";
    let string = str.to_string();
    println!("to_string {:?}", string); // to_string "abc123"

    // Возвращает минимальный по сравнению срез.
    let str1 = "abc";
    let str2 = "123";
    println!("min {:?}", str1.min(str2)); // min "123"

    // Возвращает максимальный по сравнению срез.
    let str1 = "abc";
    let str2 = "123";
    println!("max {:?}", str1.max(str2)); // max "abc"

    // Возвращает self если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. Паника если min > max.
    let str = "abc";
    println!("clamp {:?}", str.clamp("aca", "adc")); // clamp "aca"

    // Клонирует срез.
    let str1 = "abc";
    let str2 = str1.clone();
    println!("clone {:?}", str2); // clone "abc"

   // Клонирует срез в изменяемый срез.
   let str1 = "abc";
   let mut str2 = "";
   str2.clone_from(&str1);
   println!("clone_from {:?}", str2); // clone_from "abc"

   // Сравнивает строки.
   let str1 = "abc";
   let str2 = "123";
   println!("cmp {:?}", str1.cmp(str2)); // cmp Greater

}
