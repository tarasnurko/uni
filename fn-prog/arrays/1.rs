fn max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut current_count = 0;

    for &num in nums.iter() {
        if num == 1 {
            current_count += 1;
            max_count = max_count.max(current_count);
        } else {
            current_count = 0;
        }
    }

    max_count
}

fn main() {
    let nums1 = vec![1, 1, 0, 1, 1, 1];
    let result1 = max_consecutive_ones(nums1);
    println!("Output: {}", result1); // 3

    let nums2 = vec![1, 0, 1, 1, 0, 1];
    let result2 = max_consecutive_ones(nums2);
    println!("Output: {}", result2); // 2
}
