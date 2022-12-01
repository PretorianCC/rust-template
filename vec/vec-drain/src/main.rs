fn main() {
    
    // Возвращает элементы итератора в виде фрагмента вектора.
    let mut vec = vec![1, 2, 3];
    let drain = vec.drain(..);
    println!("as_slice {:?}", drain.as_slice()); // as_slice [1, 2, 3]

}
