// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // input.chars()不能使用 []进行索引，只可以使用 chars()进行迭代
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {    
            //只将第一个大写，剩余的当作&str返回
            // collect就是一个消费者适配器，
            // 使用它可以将一个迭代器中的元素收集到指定类型中，这里我们为 v2 标注了 Vec<_> 类型，就是为了告诉 collect：请把迭代器中的元素消费掉，然后把值收集成 Vec<_> 类型
            let ans: String = first.to_uppercase().collect();
            ans + c.as_str()
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    for word in words {
        ans.push(capitalize_first(*word));
    }
    ans
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut ans = String::new();
    for word in words {
        ans.push_str(capitalize_first(*word).as_str());
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
