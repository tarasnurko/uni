fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }

    let mut unique_count = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            nums[unique_count] = nums[i];
            unique_count += 1;
        }
    }
    nums.truncate(unique_count);
    unique_count
}

fn main() {
    let mut nums1 = vec![1, 1, 2];
    let result1 = remove_duplicates(&mut nums1);
    println!("Output: {}, nums = {:?}", result1, nums1); // 2, nums = [1, 2]

    let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result2 = remove_duplicates(&mut nums2);
    println!("Output: {}, nums = {:?}", result2, nums2); // 5, nums = [0, 1, 2, 3, 4]
}
