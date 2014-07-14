fn is_perfect(num: int) -> bool {
    let root = (num as f32).sqrt();
    return root.fract() == 0f32;
}

fn main() {
  for a in range(100i, 500) {
      for b in range(100i, 500) {
          let prod = a*a + b*b;
          if is_perfect(prod) {
              let c = (prod as f32).sqrt() as int;
              if a+b+c == 1000 {
                  println!("Got it: {:d}*{:d}*{:d}={:d}",a,b,c,(a*b*c));
              }
          }
      }
  }
}
