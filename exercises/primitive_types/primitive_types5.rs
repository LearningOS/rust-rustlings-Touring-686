// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.


fn main() {
    let cat = ("Furry McFurson", 3.5);
    // 元组中对应的值会绑定到变量 x， y， z上
    let (name,age) = cat;

    println!("{} is {} years old.", name, age);
}
