fn main() {
    
    // Перемещает все элементы вектора в другой вектор.
    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec1.append(&mut vec2);
    println!("append {:?} {:?}", vec1, vec2); // append [1, 2, 3, 4, 5, 6] []

    // Возвращает небезопасный изменяемый указатель на буфер вектора.
    let mut vec = vec![1, 2, 3];
    let vec_ptr = vec.as_mut_ptr();
    println!("as_mut_ptr {:?}", vec_ptr); // as_mut_ptr 0x1855920e020

    // Извлекает изменяемый срез всего вектора. Аналог &mut s[..].
    let mut vec = vec![1, 2, 3];
    let vec_slice = vec.as_mut_slice();
    println!("as_mut_slice {:?}", vec_slice); // as_mut_slice [1, 2, 3]

    // Возвращает необработанный не изменяемый указатель в буфер вектора.
    let vec = vec![1, 2, 3];
    let vec_ptr = vec.as_ptr();
    println!("as_ptr {:?}", vec_ptr); // as_ptr 0x255e4e2df20

    // Извлекает срез, содержащий весь вектор. Аналог &s[..].
    let vec = vec![1, 2, 3];
    let buffer = vec.as_slice();
    println!("as_slice {:?}", buffer); // as_slice [1, 2, 3]

    // Возвращает число элементов, которое вектор может удерживать без перераспределения памяти.
    let vec = vec![1, 2, 3];
    let size = vec.capacity();
    println!("capacity {:?}", size); // capacity 3

    // Удаляет все значения из вектора.
    let mut vec = vec![1, 2, 3];
    vec.clear();
    println!("clear {:?}", vec); // clear []

    // Если вектор отсортирован, все дубликаты будут удалены.
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    println!("dedup {:?}", vec); // dedup [1, 2, 3, 2]

    // Если вектор отсортирован, удаляет все дубликаты вектора, удовлетворяющие заданному уравнению равенства, кроме первого.
    let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
    vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
    println!("dedup_by {:?}", vec); // dedup_by ["foo", "bar", "baz", "bar"]

    // Если вектор отсортирован, удаляет все дубликаты вектора, удовлетворяющие заданному результату, кроме первого.
    let mut vec = vec![10, 20, 21, 30, 20];
    vec.dedup_by_key(|i| *i / 10);
    println!("dedup_by_key {:?}", vec); // dedup_by_key [10, 20, 30, 20]

    // Переносит указанный диапазон из вектора в другой вектор.
    let mut vec = vec![1, 2, 3];
    let buffer: Vec<_> = vec.drain(1..).collect();
    println!("drain {:?} {:?}", vec, buffer); // drain [1] [2, 3]

    // Клонирует второй вектор и переносит значения в первый.
    let mut vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    vec1.extend_from_slice(&vec2);
    println!("extend_from_slice {:?} {:?}", vec1, vec2); // extend_from_slice [1, 2, 3, 4, 5, 6] [4, 5, 6]

    // Добавляет в вектор копированием срез с этого же вектора.
    let mut vec = vec![1, 2, 3];
    vec.extend_from_within(1..);
    println!("extend_from_within {:?}", vec); // extend_from_within [1, 2, 3, 2, 3]

    // Создание вектора из необработанных компонентов другого вектора.
    let mut vec1 = vec![1, 2, 3];
    let p = vec1.as_mut_ptr();
    unsafe {
        let vec2 = Vec::from_raw_parts(p, vec1.len(), 3);
        println!("Vec::from_raw_parts {:?} {:?}", vec1, vec2); // Vec::from_raw_parts [1, 2, 3] [1, 2, 3]
    }

    // Добавляет в вектор значения по индексу.
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    println!("insert {:?}", vec); // insert [1, 4, 2, 3]

    // Преобразует вектор в Box.
    let vec = vec![1, 2, 3];
    let slice = vec.into_boxed_slice();
    println!("into_boxed_slice {:?}", slice); // into_boxed_slice [1, 2, 3]

    // Проверяет пустой ли вектор.
    let vec = vec![1, 2, 3];
    println!("is_empty {:?}", vec.is_empty()); // is_empty false

    // Возвращает изменяемую ссылку на указатель.
    let vec = vec![1, 2, 3];
    let ptr = vec.leak();
    println!("leak {:?}", ptr); // leak [1, 2, 3]

    // Число элементов в векторе.
    let vec = vec![1, 2, 3];
    println!("len {:?}", vec.len()); // len 3

    // Новый вектор.
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vec::new {:?}", vec); // Vec::new [1, 2, 3]

    // Удаляет последний элемент из вектора и возвращает его или None, если он пуст.
    let mut vec = vec![1, 2, 3];
    let value = vec.pop();
    println!("pop {:?} {:?}", value, vec); // pop Some(3) [1, 2]

    // Добавляет элемент в конец вектора.
    let mut vec = Vec::new();
    vec.push(1);
    println!("push {:?}", vec); // push [1]

    // Удаляет и возвращает элемент по индексу в векторе, сдвигая все элементы после него влево.
    let mut vec = vec![1, 2, 3];
    let value = vec.remove(1);
    println!("remove {value} {:?}", vec); // remove 2 [1, 3]

    // Резервирует емкость вектора для дополнительных элементов.
    let mut vec = vec![1, 2, 3];
    vec.reserve(10);
    println!("reserve {}", vec.capacity()); // reserve 13

    // Резервирует минимальную емкость вектора для дополнительных элементов. В отличие от reserve, не будет преднамеренно чрезмерно распределять, чтобы избежать частых распределений. 
    let mut vec = vec![1, 2, 3];
    vec.reserve_exact(10);
    println!("reserve_exact {}", vec.capacity()); // reserve_exact 13

    // Изменяет размер вектора увеличивая до указанного размера, инициализируя новые значения указанным значением.
    let mut vec = vec![1, 2, 3];
    vec.resize(5, 0);
    println!("resize {:?}", vec); // resize [1, 2, 3, 0, 0]

    // Изменяет размер вектора увеличивая до указанного размера, инициализируя новые значения из функции.
    let mut vec = vec![1, 2, 3];
    let mut p = 1;
    vec.resize_with(5, || { p *= 2; p });
    println!("resize_with {:?}", vec); // resize_with [1, 2, 3, 2, 4]

    // Сохраняет элементы которые true после обработки функцией.
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    vec.retain(|&x| x % 2 == 0);
    println!("retain {:?}", vec); // retain [2, 4, 6]

    // Сохраняет элементы которые true после обработки функцией c возможностью изменения элементов.
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    vec.retain_mut(|x| if *x <= 3 {
            *x += 1;
            true
        } else {
            false
        });
    println!("retain_mut {:?}", vec); // retain_mut [2, 3, 4]

    // Принудительно изменяет длину вектора.
    let mut vec = vec![1, 2, 3];
    unsafe {
        vec.set_len(1);
    }
    println!("set_len {:?}", vec); // set_len [1]

    // Уменьшает емкость вектора до указанного или возможного.
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    vec.shrink_to(2);
    println!("shrink_to {}", vec.capacity()); // shrink_to 3

    // Максимально уменьшает емкость вектора.
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3]);
    vec.shrink_to_fit();
    println!("shrink_to_fit {}", vec.capacity()); // shrink_to_fit 3

    // Возвращает остающуюся резервную мощность вектора как MaybeUninit<T>.
    let mut vec = Vec::with_capacity(10);
    let uninit = vec.spare_capacity_mut();
    uninit[0].write(0);
    uninit[1].write(1);
    uninit[2].write(2);
    unsafe {
        vec.set_len(3);
    }
    println!("spare_capacity_mut {:?}", vec); // spare_capacity_mut [0, 1, 2]

    // Заменяет значения в векторе в диапазоне, возвращая замененные значения.
    let mut vec = vec![1, 2, 3];
    let arr = [7, 8, 9];
    let new_vec: Vec<_> = vec.splice(1..3, arr).collect();
    println!("splice {:?} {:?}", vec, new_vec); // splice [1, 7, 8, 9] [2, 3]

    // Разбивает вектор на два по индексу.
    let mut vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.split_off(2);
    println!("split_off {:?} {:?}", vec1, vec2); // split_off [1, 2] [3, 4, 5]

    // Заменяет элемент по индексу последним элементом, перемещая его.
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.swap_remove(2);
    println!("swap_remove {:?}", vec); // swap_remove [1, 2, 5, 4]

    // Укорачивает вектор.
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.truncate(2);
    println!("truncate {:?}", vec); // truncate [1, 2]

    // Пытается зарезервировать емкость для дополнительных элементов.
    let mut vec: Vec<i32> = Vec::new();
    vec.try_reserve(10).unwrap();
    println!("try_reserve {}", vec.capacity()); // try_reserve 10

    // Пытается зарезервировать минимальную емкость для дополнительных элементов.
    let mut vec: Vec<i32> = Vec::new();
    vec.try_reserve_exact(10).unwrap();
    println!("try_reserve_exact {}", vec.capacity()); // try_reserve_exact 10

    // Создает пустой вектор с указанной ёмкостью.
    let vec: Vec<i32> = Vec::with_capacity(10);
    println!("Vec::with_capacity {}", vec.capacity()); // Vec::with_capacity 10

}
