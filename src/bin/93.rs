#![feature(try_blocks)]
use itertools::Itertools;

fn calc(num1: f32, num2: f32, op: u8) -> Result<f32, ()> {
    match op {
        0 => Ok(num1 + num2),
        1 => Ok(num1 - num2),
        2 => Ok(num1 * num2),
        3 => {
            if num2 == 0.0 {
                Err(())
            } else {
                Ok(num1 / num2)
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    let mut ans_n = 0;
    let mut ans = 0;
    let mut nums = [0f32; 4];
    for a in 0..10i32 {
        for b in a + 1..10 {
            for c in b + 1..10 {
                for d in c + 1..10 {
                    let mut made = std::collections::HashSet::<i32>::new();
                    for perm in [a, b, c, d].iter().permutations(4) {
                        for pos1 in 0..3 {
                            for op1 in 0..4 {
                                for pos2 in 0..2 {
                                    for op2 in 0..4 {
                                        for op3 in 0..4 {
                                            nums[0] = *perm[0] as f32;
                                            nums[1] = *perm[1] as f32;
                                            nums[2] = *perm[2] as f32;
                                            nums[3] = *perm[3] as f32;

                                            let _: Result<(), ()> = try {
                                                nums[pos1] = calc(nums[pos1], nums[pos1 + 1], op1)?;
                                                for i in pos1 + 1..3 {
                                                    nums[i] = nums[i + 1];
                                                }
                                                nums[pos2] = calc(nums[pos2], nums[pos2 + 1], op2)?;
                                                for i in pos2 + 1..2 {
                                                    nums[i] = nums[i + 1];
                                                }
                                                nums[0] = calc(nums[0], nums[1], op3)?;

                                                let num: i32 = nums[0].round() as i32;

                                                if ((num as f32) - nums[0]).abs() < 1e-5 {
                                                    made.insert(num);
                                                }
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let mut last_n = 0;
                    for n in 1.. {
                        if !made.contains(&n) {
                            break;
                        }
                        last_n = n;
                    }
                    if last_n > ans_n {
                        ans_n = last_n;
                        ans = a * 1000 + b * 100 + c * 10 + d;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
