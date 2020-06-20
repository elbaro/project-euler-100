use rayon::prelude::*;

const M: usize = 1_000_000;

fn main() {
    let next: Vec<usize> = (0..=M)
        .into_par_iter()
        .map(|i| {
            let mut sum = 0usize;
            for j in 1..i {
                if j * j > i {
                    break;
                }
                if i % j == 0 {
                    if j == 1 || j * j == i {
                        sum += j;
                    } else {
                        sum += j + i / j;
                    }
                }
            }
            sum
        })
        .collect();

    let mut visited = vec![0usize; M + 1];
    let mut stack = Vec::new();

    let mut mx_len = 0usize;
    let mut mn_num = 0usize;
    for i in 1..=M {
        if visited[i] == 0 {
            stack.clear();
            let mut num = i;
            loop {
                stack.push(num);
                visited[num] = i;
                if next[num] > M {
                    break;
                }
                match visited[next[num]] {
                    0 => {
                        num = next[num];
                    }
                    v if (v == i) => {
                        let start_idx = stack.iter().position(|&x| x == next[num]).unwrap();
                        let len: usize = stack[start_idx..].len();
                        let mn: usize = *stack[start_idx..].iter().min().unwrap();
                        if len > mx_len {
                            mx_len = len;
                            mn_num = mn;
                        } else if len == mx_len && mn < mn_num {
                            mn_num = mn;
                        }

                        break;
                    }
                    _ => break,
                }
            }
        }
    }
    println!("{}", mn_num);
}
