use crate::types::integer::Integer;

pub fn calc_prime_divisors<T: Integer>(v: T) -> PrimeDivisorsIter<T> {
    if v <= T::from(0) {
        panic!("Cannot calculate prime divisors for {v:?}, only positive numbers are supported")
    }
    PrimeDivisorsIter {
        value: v,
        rem: v,
        next: T::from(2),
    }
}

pub struct PrimeDivisorsIter<T> {
    value: T,
    rem: T,
    next: T,
}

impl<T: Integer> Iterator for PrimeDivisorsIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (zero, one) = (T::from(0), T::from(1));
        while self.next * self.next <= self.value {
            let cur = self.next;
            self.next += one;
            if self.rem % cur == zero {
                while self.rem % cur == zero {
                    self.rem /= cur;
                }
                return Some(cur);
            }
        }
        if self.rem > one {
            let cur = self.rem;
            self.rem = one;
            Some(cur)
        } else {
            None
        }
    }
}

pub fn calc_all_divisors<T: Integer>(v: T) -> AllDivisorsIter<T> {
    if v <= T::from(0) {
        panic!("Cannot calculate divisors for {v:?}, only positive numbers are supported",)
    }
    AllDivisorsIter {
        value: v,
        next: T::from(1),
        take_other: false,
    }
}

pub struct AllDivisorsIter<T> {
    value: T,
    next: T,
    take_other: bool,
}

impl<T: Integer> Iterator for AllDivisorsIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (zero, one) = (T::from(0), T::from(1));
        if self.take_other {
            let cur = self.next;
            let other = self.value / cur;
            self.next += one;
            self.take_other = false;
            if other > cur {
                Some(other)
            } else {
                None
            }
        } else {
            loop {
                if self.next * self.next > self.value {
                    return None;
                }
                if self.value % self.next == zero {
                    break;
                }
                self.next += one;
            }
            self.take_other = true;
            Some(self.next)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{calc_all_divisors, calc_prime_divisors};
    use crate::plat::classic::includes::IntoVecExt;
    use crate::types::integer::Integer;

    #[test]
    fn test_calc_prime_divisors() {
        check_prime(1, &[]);
        check_prime(2, &[2]);
        check_prime::<u8>(8, &[2]);
        check_prime::<u32>(6, &[2, 3]);
        check_prime::<usize>(26, &[2, 13]);
    }

    #[test]
    fn test_calc_all_divisors() {
        check_all(1, &[1]);
        check_all(2, &[1, 2]);
        check_all(4, &[1, 4, 2]);
        check_all::<usize>(36, &[1, 36, 2, 18, 3, 12, 4, 9, 6]);
    }

    fn check_prime<T: Integer>(v: T, expected: &[T]) {
        assert_eq!(
            calc_prime_divisors(v).into_vec(),
            expected.to_vec(),
            "{v:?}"
        );
    }

    fn check_all<T: Integer>(v: T, expected: &[T]) {
        assert_eq!(calc_all_divisors(v).into_vec(), expected.to_vec(), "{v:?}");
    }
}
