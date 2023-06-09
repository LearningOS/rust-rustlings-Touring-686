// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


fn main() {

    //可变引用同时只能存在一个
    let mut x = 100;
    let z = &mut x;
    *z += 1100;
    assert_eq!(x, 1200);
}
