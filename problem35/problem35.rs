
/*
The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?
*/

fn rotate(num: int) -> int {
  let s = num.to_str();
  let mut _s = s.as_slice();
  let len = _s.len();
  println!("num {} has len {}",num,len);

  let mut new_s = _s.slice_from(1).to_str();
  new_s.push_str(_s.slice(0,1));

  let new_num: Option<int> = from_str(new_s.as_slice());


  let the_num = match new_num {
      Some(number) => {
          let new_len = number.to_str().as_slice().len();
          // println!("new num {} has len {}",new_num,new_len);
          if new_len != len { -1 } else { number }
      }
      None         => -1
  };

  return the_num;
}


fn is_prime(num: int) -> bool {
    match num {
        2 => true,
        3 => true,
        _ => {
            let mut result:bool = true;
            let max = (num as f32).sqrt();
            let mut i = 2;
            while i <= (max as int){
                let rem = (num as f32) / (i as f32);
                if rem.fract() == 0f32 {
                    result = false;
                    break;
                }
                i = i + 1;
            }
            result
        }
    }
}

fn main() {
  let mut primes: Vec<int> = Vec::new();

  //for i in range(2i, 100i) {
  for i in range(2i, 1000000i) {
      // let mut nums: Vec<int> = Vec::new();
      println!("{}",i);
      println!("{}",rotate(i));
      let mut r = i;

      if !is_prime(i) {
          println!("Skipping {}",i);
          continue;
      }

      loop {
          r = rotate(r);

          println!("checking {} == {}",r,i);

          if r == -1 {
              break;
          }

          if !is_prime(r) {
              println!("Skipping rotated {}",r);
              break;
          }

          if r == i {
              primes.push(i);
              println!("========");
              break;
          } else {
              println!("does not match");
          }
      }
  }

  println!("GOT: {}: {}", primes, primes.len());
}
