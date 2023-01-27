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
