struct Divisors {
    n: usize,
    next: usize,
    next_large: bool,
}

fn divisors(n: usize) -> Divisors {
    Divisors {
        n: n,
        next: 1,
        next_large: false,
    }
}

impl Iterator for Divisors {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            match self.next * self.next {
                sq if sq > self.n => { return None; }
                sq if sq == self.n => {
                    if self.next_large {
                        return None;
                    }
                    self.next_large = true;
                    return Some(self.next);
                }
                _ => {
                    if self.n % self.next == 0 {
                        if self.next_large {
                            let large = self. n / self.next;
                            self.next += 1;
                            self.next_large = false;
                            return Some(large);
                        } else {
                            self.next_large = true;
                            return Some(self.next);
                        }
                    }
                    self.next += 1;
                }
            }
        }
    }
}
