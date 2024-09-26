fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    let mut s: Vec<char> = target.chars().collect();
    let stamp_chars: Vec<char> = stamp.chars().collect();
    let mut res = Vec::new();
    let mut total_stamped = 0;
    let stamp_len = stamp.len();
    let target_len = target.len();
    let mut visited = vec![false; target_len];

    while total_stamped < target_len {
        let mut stamped = false;
        for i in 0..=target_len - stamp_len {
            if !visited[i] && can_stamp(&s, i, &stamp_chars) {
                total_stamped += do_stamp(&mut s, i, stamp_len);
                visited[i] = true;
                stamped = true;
                res.push(i as i32);
                if total_stamped == target_len {
                    break;
                }
            }
        }
        if !stamped {
            return vec![];
        }
    }

    res.reverse();
    res
}

fn can_stamp(s: &Vec<char>, pos: usize, stamp: &Vec<char>) -> bool {
    let mut can_stamp = false;
    for i in 0..stamp.len() {
        if s[pos + i] == '?' {
            continue;
        }
        if s[pos + i] != stamp[i] {
            return false;
        }
        can_stamp = true;
    }
    can_stamp
}

fn do_stamp(s: &mut Vec<char>, pos: usize, stamp_len: usize) -> usize {
    let mut stamped = 0;
    for i in 0..stamp_len {
        if s[pos + i] != '?' {
            s[pos + i] = '?';
            stamped += 1;
        }
    }
    stamped
}

fn main() {
    let stamp1 = "abc".to_string();
    let target1 = "ababc".to_string();
    let result1 = moves_to_stamp(stamp1, target1);
    println!("{:?}", result1); // [0, 2]

    let stamp2 = "abca".to_string();
    let target2 = "aabcaca".to_string();
    let result2 = moves_to_stamp(stamp2, target2);
    println!("{:?}", result2); // [3, 0, 1]
}
