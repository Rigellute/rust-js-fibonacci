use std::collections::HashMap;

pub fn fibonacci_recursive(num: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if num <= 2 {
        return 1;
    }

    if let Some(val) = memo.get(&num) {
        return *val;
    }

    let value = fibonacci_recursive(num - 1, memo) + fibonacci_recursive(num - 2, memo);
    memo.insert(num, value);

    return value;
}

pub fn fibonacci(num: i64) -> i64 {
    let mut memo: HashMap<i64, i64> = HashMap::new();
    return fibonacci_recursive(num, &mut memo);
}

#[test]
fn it_works() {
    let first = fibonacci(1);
    assert_eq!(first, 1);
}
