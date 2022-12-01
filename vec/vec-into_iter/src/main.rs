fn main() {
    
    // Возвращает элементы итератора в виде изменяемого фрагмента вектора.
    let vec = vec![1, 2, 3];
    let mut into_iter = vec.into_iter();
    println!("as_mut_slice {:?}", into_iter.as_mut_slice()); // as_mut_slice [1, 2, 3]

    // Возвращает элементы итератора в виде фрагмента вектора.
    let vec = vec![1, 2, 3];
    let into_iter = vec.into_iter();
    println!("as_slice {:?}", into_iter.as_slice()); // as_slice [1, 2, 3]
}
