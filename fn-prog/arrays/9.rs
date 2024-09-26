fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    let mut max_from_right = -1;

    for i in (0..arr.len()).rev() {
        let current = arr[i];
        arr[i] = max_from_right;
        if current > max_from_right {
            max_from_right = current;
        }
    }

    arr
}

fn main() {
    let arr1 = vec![17, 18, 5, 4, 6, 1];
    let result1 = replace_elements(arr1);
    println!("Output: {:?}", result1); // [18, 6, 6, 6, 1, -1]

    let arr2 = vec![400];
    let result2 = replace_elements(arr2);
    println!("Output: {:?}", result2); // [-1]
}
