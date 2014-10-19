struct Collatz { current: int }
impl Iterator<int> for Collatz {
    fn next(&mut self) -> Option<int> {
        if self.current == 1 {
            None
        } else {
            self.current = if self.current % 2 == 0 {
                self.current / 2
            } else {
                3*self.current + 1
            };

            Some(self.current)
        }
    }
}


fn main() {
    let max = 1000000;
    let mut max_start = 0;
    let mut max_len = 0;

    for i in range(1i, max) {
        let mut col = Collatz { current: i };
        let items: Vec<int> = col.collect();
        let len = items.len();
        if len > max_len {
            max_len = len;
            max_start = i;
        }
        println!("{} -> {}",i,len);
    }

    println!("{} -> max len: {}",max_start,max_len);
}
