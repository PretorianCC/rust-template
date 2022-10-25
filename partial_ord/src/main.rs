use std::cmp::PartialOrd;

fn main() {
    
    // Сравнивает значения.
    let result = 1.partial_cmp(&2);
    println!("partial_cmp {:?}", result); // partial_cmp Some(Less)

    // Оператор меньше.
    let result = 1 < 2;
    println!("operator < {:?}", result); // operator < true

    // Оператор больше.
    let result = 1 > 2;
    println!("operator > {:?}", result); // operator > false

    // Оператор меньше или равно.
    let result = 1 <= 2;
    println!("operator <= {:?}", result); // operator <= true

    // Оператор больше или равно.
    let result = 1 >= 2;
    println!("operator >= {:?}", result); // operator >= false
}
