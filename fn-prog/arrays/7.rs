fn check_double_exists(arr: Vec<i32>) -> bool {
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i != j && arr[i] == 2 * arr[j] {
                return true;
            }
        }
    }
    false
}

fn main() {
    let arr1 = vec![10, 2, 5, 3];
    let result1 = check_double_exists(arr1);
    println!("Output: {}", result1); // true

    let arr2 = vec![3, 1, 7, 11];
    let result2 = check_double_exists(arr2);
    println!("Output: {}", result2); // false
}
