fn max_sliding_window(nums: Vec<i32>, k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return vec![];
    }

    let mut result = Vec::new();
    let mut deque: Vec<usize> = Vec::new();

    for i in 0..nums.len() {
        if !deque.is_empty() && deque[0] <= i.saturating_sub(k) {
            deque.remove(0);
        }
        while !deque.is_empty() && nums[*deque.last().unwrap()] < nums[i] {
            deque.pop();
        }

        deque.push(i);

        if i >= k - 1 {
            result.push(nums[deque[0]]);
        }
    }

    result
}

fn main() {
    let nums1 = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k1 = 3;
    let result1 = max_sliding_window(nums1, k1);
    println!("{:?}", result1); // [3, 3, 5, 5, 6, 7]

    let nums2 = vec![1];
    let k2 = 1;
    let result2 = max_sliding_window(nums2, k2);
    println!("{:?}", result2); // [1]
}
