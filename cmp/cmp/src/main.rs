use std::cmp;

fn main() {
    
    // Возвращает максимальное значение, если они равны возвращает второй аргумент.
    let result = cmp::max(1, 2);
    println!("max {}", result); // max 2

    // Возвращает вычисленное в функции максимальное значение.
    let result = cmp::max_by(1, 2, |a, b| a.cmp(&b));
    println!("max_by {}", result); // max_by 2

    // Возвращает первый параметр если возвращаемое значение в функции больше второго параметра, иначе возвращает второй параметр.
    let result = cmp::max_by_key(-3, 2, |a: &i32| a.abs());
    println!("max_by_key {}", result); // max_by_key -3

    // Возвращает минимальное значение, если они равны возвращает второй аргумент.
    let result = cmp::min(1, 2);
    println!("min {}", result); // min 1

    // Возвращает вычисленное в функции минимальное значение.
    let result = cmp::min_by(1, 2, |a, b| a.cmp(&b));
    println!("min_by {}", result); // min_by 1

    // Возвращает первый параметр если возвращаемое значение в функции меньше второго параметра, иначе возвращает второй параметр.
    let result = cmp::min_by_key(3, -2, |a: &i32| -a);
    println!("min_by_key {}", result); // min_by_key 3

}
