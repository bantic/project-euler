extern crate std;

pub struct Natural { current: int }
impl Natural {
    pub fn new() -> Natural {
        Natural { current: 1 }
    }
}
impl Iterator<int> for Natural {
    fn next(&mut self) -> Option<int> {
        self.current = self.current + 1;

        Some(self.current)
    }
}

pub struct Primes { current: int }

impl Primes {
    pub fn new() -> Primes {
        Primes { current: 2 }
    }
    pub fn is_prime(x:int) -> bool {
        if x < 0 { return false; }
        match x {
            0 => false,
            1 => false,
            2 => true,
            _ => {
                let f = x.to_f64().unwrap();
                let sqrt = f.sqrt();
                let max = sqrt.floor().to_uint().unwrap();
                for i in std::iter::count(2i, 1i).take(max) {
                    if x % i == 0 { return false; }
                }

                true
            }
        }
    }
}

impl Iterator<int> for Primes {
    fn next(&mut self) -> Option<int> {
        let mut tmp = self.current;
        loop {
            if Primes::is_prime(tmp) { break }
            tmp = tmp + 1;
        }

        self.current = tmp + 1;
        Some(tmp)
    }
}
