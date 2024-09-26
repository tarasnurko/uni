fn merge(nums1: &mut Vec<i32>, m: usize, nums2: Vec<i32>, n: usize) {
    let mut i = m + n;
    let mut p1 = m;
    let mut p2 = n;

    while p2 > 0 {
        if p1 == 0 || nums1[p1 - 1] <= nums2[p2 - 1] {
            nums1[i - 1] = nums2[p2 - 1];
            p2 -= 1;
        } else {
            nums1[i - 1] = nums1[p1 - 1];
            p1 -= 1;
        }
        i -= 1;
    }
}

fn main() {
    let mut nums1_1 = vec![1, 2, 3, 0, 0, 0];
    merge(&mut nums1_1, 3, vec![2, 5, 6], 3);
    println!("Output: {:?}", nums1_1); // [1, 2, 2, 3, 5, 6]

    let mut nums1_2 = vec![1];
    merge(&mut nums1_2, 1, vec![], 0);
    println!("Output: {:?}", nums1_2); // [1]

    let mut nums1_3 = vec![0];
    merge(&mut nums1_3, 0, vec![1], 1);
    println!("Output: {:?}", nums1_3); // [1]
}
