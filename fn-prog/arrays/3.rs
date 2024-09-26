fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut squares: Vec<i32> = nums.iter().map(|&num| num * num).collect();
    squares.sort();
    squares
}

fn main() {
    let nums1 = vec![-4, -1, 0, 3, 10];
    let result1 = sorted_squares(nums1);
    println!("Output: {:?}", result1); // [0, 1, 9, 16, 100]

    let nums2 = vec![-7, -3, 2, 3, 11];
    let result2 = sorted_squares(nums2);
    println!("Output: {:?}", result2); // [4, 9, 9, 49, 121]
}
