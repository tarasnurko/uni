fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_by_key(|&x| x % 2);
    nums
}

fn main() {
    let nums1 = vec![3, 1, 2, 4];
    let result1 = sort_array_by_parity(nums1);
    println!("Output: {:?}", result1); // [2, 4, 3, 1]

    let nums2 = vec![0];
    let result2 = sort_array_by_parity(nums2);
    println!("Output: {:?}", result2); // [0]
}
