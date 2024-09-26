fn count_numbers_with_even_digits(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for &num in nums.iter() {
        if (num.to_string().len() % 2) == 0 {
            count += 1;
        }
    }
    count
}

fn main() {
    let nums1 = vec![12, 345, 2, 6, 7896];
    let result1 = count_numbers_with_even_digits(nums1);
    println!("Output: {}", result1); // 2

    let nums2 = vec![555, 901, 482, 1771];
    let result2 = count_numbers_with_even_digits(nums2);
    println!("Output: {}", result2); // 1
}
