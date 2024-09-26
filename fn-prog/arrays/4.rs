fn duplicate_zeros(arr: &mut Vec<i32>) {
    let len = arr.len();
    let mut count_zeros = 0;

    for &num in arr.iter() {
        if num == 0 {
            count_zeros += 1;
        }
    }

    let mut i = len - 1;

    while i != 0 {
        if arr[i] == 0 {
            if i + count_zeros < len {
                arr[i + count_zeros] = 0;
            }
            count_zeros -= 1;
            if i + count_zeros < len {
                arr[i + count_zeros] = 0;
            }
        } else {
            if i + count_zeros < len {
                arr[i + count_zeros] = arr[i];
            }
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
}

fn main() {
    let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
    duplicate_zeros(&mut arr1);
    println!("Output: {:?}", arr1); // [1, 0, 0, 2, 3, 0, 0, 4]

    let mut arr2 = vec![1, 2, 3];
    duplicate_zeros(&mut arr2);
    println!("Output: {:?}", arr2); // [1, 2, 3]
}
