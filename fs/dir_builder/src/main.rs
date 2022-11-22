use std::fs::DirBuilder;

fn main() {
    
    // Создание нового набора параметров для каталога.
    let builder = DirBuilder::new();
    println!("DirBuilder::new {:?}", builder); // DirBuilder::new DirBuilder { inner: DirBuilder, recursive: false }

    // Указывает, что каталоги должны создаваться рекурсивно.fn
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    println!("recursive {:?}", builder); // recursive DirBuilder { inner: DirBuilder, recursive: true }

    // Создает указанный каталог.
    let path = "./tmp/foo";
    let mut builder = DirBuilder::new();
    builder.recursive(true);
    let result = builder.create(path);
    println!("create {:?}", result); // create Ok(())

}
