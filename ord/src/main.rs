use std::cmp::Ord;

fn main() {

    // Сравнивает значения.
    let result = 1.cmp(&2);
    println!("cmp {:?}", result); // cmp Less

    // Возвращает максимальное значение.
    let result = 1.max(2);
    println!("max {:?}", result); // max 2

    // Возвращает минимальное значение.
    let result = 1.min(2);
    println!("min {:?}", result); // min 1

    // Возвращает self если он входит в диапазон min-max, иначе возвращает минимальное или максимальное. Паника если min > max.
    let result = 1.clamp(2, 3);
    println!("clamp {:?}", result); // clamp 2


}
