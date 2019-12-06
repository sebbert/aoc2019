pub struct ReverseDigits {
    pub radix: isize,
    pub value: isize,
}

impl ReverseDigits {
    pub fn new(value: isize, radix: isize) -> Self {
        ReverseDigits { value, radix }
    }
}

impl Iterator for ReverseDigits {
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        match self.value {
            0 => None,
            value => {
                let next = value % self.radix;
                self.value /= self.radix;
                Some(next)
            }
        }
    }
}

#[test]
fn test_digits_base_10() {
    let digits = ReverseDigits::new(12345, 10).collect::<Vec<_>>();
    assert!(digits == vec![5,4,3,2,1])
}

#[test]
fn test_digits_base_2() {
    let digits = ReverseDigits::new(0b11011101001, 2).collect::<Vec<_>>();
    assert!(digits == vec![1,0,0,1,0,1,1,1,0,1,1])
}

#[test]
fn test_digits_base_100() {
    let digits = ReverseDigits::new(123456, 100).collect::<Vec<_>>();
    assert!(digits == vec![56,34,12])
}
