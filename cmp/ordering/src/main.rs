use std::cmp::Ordering;

fn main() {

    // Cравниваемое значение меньше другого.
    let result = 1.cmp(&2); // 1 < 2 Less 
    println!("1 < 2 {:?} ", result);

    // Сравниваемое значение равно другому.
    let result = 1.cmp(&1);
    println!("1 = 1 {:?} ", result); // 1 = 1 Equal

    // Сравниваемое значение больше другого.
    let result = 2.cmp(&1);
    println!("2 > 1 {:?} ", result); // 2 > 1 Greater

    // Это Ordering::Equal?
    println!("Ordering::Equal {:?} ", Ordering::Equal.is_eq()); // Ordering::Equal true

    // Это не Ordering::Equal?
    println!("Ordering::Equal {:?} ", Ordering::Equal.is_ne()); // Ordering::Equal false

    // Это Ordering::Less?
    println!("Ordering::Less {:?} ", Ordering::Less.is_lt()); // Ordering::Less true 

    // Это Ordering::Greater?
    println!("Ordering::Greater {:?} ", Ordering::Greater.is_gt()); // Ordering::Greater true 

    // Это Ordering::Less или Ordering::Equal?
    println!("Ordering::Less, Ordering::Equal {:?} ", Ordering::Greater.is_le()); // Ordering::Less, Ordering::Equal false 

    // Это Ordering::Greater или Ordering::Equal?
    println!("Ordering::Greater, Ordering::Equal {:?} ", Ordering::Less.is_ge()); // Ordering::Greater, Ordering::Equal false 

    // Противоположное значение. Ordering::Greater в Ordering::Less, Ordering::Less в Ordering::Greater, Ordering::Equal в Ordering::Equal.
    println!("Ordering::Greater {:?} ", Ordering::Greater.reverse()); // Ordering::Greater Less 
    println!("Ordering::Less {:?} ", Ordering::Less.reverse()); // Ordering::Less Greater 
    println!("Ordering::Equal {:?} ", Ordering::Equal.reverse()); // Ordering::Equal Equal

    // Вернуть Less или Greater с Equal, иначе self.
    println!("Ordering::Equal Less {:?} ", Ordering::Equal.then(Ordering::Less)); // Less
    println!("Ordering::Less Equal {:?} ", Ordering::Less.then(Ordering::Equal)); // Less
    println!("Ordering::Less Greater {:?} ", Ordering::Less.then(Ordering::Greater)); // Less
    println!("Ordering::Equal Greater {:?} ", Ordering::Equal.then(Ordering::Greater)); // Greater
    println!("Ordering::Greater Equal {:?} ", Ordering::Greater.then(Ordering::Equal)); // Greater
    println!("Ordering::Greater Less {:?} ", Ordering::Greater.then(Ordering::Greater)); // Greater
    println!("Ordering::Equal Equal {:?} ", Ordering::Equal.then(Ordering::Equal)); // Equal

    // Вернуть Less или Greater с Equal, иначе self. Вариант с функцией.
    println!("Ordering::Equal Less {:?} ", Ordering::Equal.then_with(|| Ordering::Less)); // Less
    println!("Ordering::Less Equal {:?} ", Ordering::Less.then_with(|| Ordering::Equal)); // Less
    println!("Ordering::Less Greater {:?} ", Ordering::Less.then_with(|| Ordering::Greater)); // Less
    println!("Ordering::Equal Greater {:?} ", Ordering::Equal.then_with(|| Ordering::Greater)); // Greater
    println!("Ordering::Greater Equal {:?} ", Ordering::Greater.then_with(|| Ordering::Equal)); // Greater
    println!("Ordering::Greater Less {:?} ", Ordering::Greater.then_with(|| Ordering::Greater)); // Greater
    println!("Ordering::Equal Equal {:?} ", Ordering::Equal.then_with(|| Ordering::Equal)); // Equal

}
