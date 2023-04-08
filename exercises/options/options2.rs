// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let

        // vec.pop在Option<>外又加了一层Option 而且Some是可以嵌套的
        // while的推出条件是什么？返回是pop返回None
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
