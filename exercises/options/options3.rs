// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

// 具体来说，当我们使用match表达式将可选值解包时，如果匹配到了Some分支，
// 则该值的所有权将被移动到新的变量中，此时原来的可选值变量y已经不再拥有该值的所有权，
// 而当匹配到None分支时，y仍然拥有原始值的所有权。
// 我们再次使用变量y来尝试访问值的字段，因为所有权已经被移动到新的变量中。
// 所有权被移动到 p中
    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y), // use of partially moved value: `y`
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
