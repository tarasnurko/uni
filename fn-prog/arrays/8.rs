fn is_mountain_array(arr: Vec<i32>) -> bool {
    if arr.len() < 3 {
        return false;
    }

    let mut i = 0;

    while i + 1 < arr.len() && arr[i] < arr[i + 1] {
        i += 1;
    }

    if i == 0 || i == arr.len() - 1 {
        return false;
    }

    while i + 1 < arr.len() && arr[i] > arr[i + 1] {
        i += 1;
    }

    i == arr.len() - 1
}

fn main() {
    let arr1 = vec![2, 1];
    let result1 = is_mountain_array(arr1);
    println!("Output: {}", result1); // false

    let arr2 = vec![3, 5, 5];
    let result2 = is_mountain_array(arr2);
    println!("Output: {}", result2); // false

    let arr3 = vec![0, 3, 2, 1];
    let result3 = is_mountain_array(arr3);
    println!("Output: {}", result3); // true
}
