use std::iter::AdditiveIterator;

struct SpiralDiagonal { size: uint, side: uint, cur: uint, max_size: uint }
impl SpiralDiagonal {
    fn new(start: uint, max_size:uint) -> SpiralDiagonal {
        SpiralDiagonal {size:1, side:1, cur:start, max_size:max_size }
    }
}

impl Iterator<uint> for SpiralDiagonal {
    fn next(&mut self) -> Option<uint> {
        if self.size > self.max_size {
            return None;
        }


        if self.size == 1 {
            self.size = 3;
        } else {
            let step = self.size-1;
            self.cur = self.cur + step;
            self.side += 1;

            if self.side > 4 {
                self.side = 1;
                self.size += 2;
            }
        }

        Some(self.cur)

    }
}

fn main() {
    let mut diag = SpiralDiagonal::new(1u, 1001u);
    let sum = diag.sum();

    println!("sum: {}",sum);
}
