fn constrained_subset_sum(nums: Vec<i32>, k: usize) -> i32 {
    let mut dp: Vec<i32> = vec![0; nums.len()];
    let mut deque: Vec<usize> = Vec::new();
    let mut max_sum = nums[0];

    for i in 0..nums.len() {
        dp[i] = nums[i];
        if let Some(&j) = deque.first() {
            dp[i] += dp[j].max(0);
        }
        max_sum = max_sum.max(dp[i]);

        while let Some(&j) = deque.last() {
            if dp[j] <= dp[i] {
                deque.pop();
            } else {
                break;
            }
        }

        deque.push(i);

        if let Some(&j) = deque.first() {
            if i >= j + k {
                deque.remove(0);
            }
        }
    }

    max_sum
}

fn main() {
    let nums1 = vec![10, 2, -10, 5, 20];
    let k1 = 2;
    let result1 = constrained_subset_sum(nums1, k1);
    println!("{}", result1); // 37

    let nums2 = vec![-1, -2, -3];
    let k2 = 1;
    let result2 = constrained_subset_sum(nums2, k2);
    println!("{}", result2); // -1

    let nums3 = vec![10, -2, -10, -5, 20];
    let k3 = 2;
    let result3 = constrained_subset_sum(nums3, k3);
    println!("{}", result3); // 23
}
