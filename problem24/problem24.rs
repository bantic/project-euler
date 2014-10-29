/*

0123456789
0123456798

0123456879
0123456897

9,123,456,870

 */

struct Lexicographer { current: Vec<uint> }
impl Lexicographer {
    fn new(v:Vec<uint>) -> Lexicographer {
        Lexicographer {current:v}
    }
}

impl Iterator<Vec<uint>> for Lexicographer {
    fn next(&mut self) -> Option<Vec<uint>> {
        let mut v = self.current.to_vec();
        let mut i = 0u; // first index that has a greater value right after it
        let mut j = 0u; // first index that has a value greater than i
        let mut k = 0u; // last index that has a greater value than i

        let mut found = false;

        let mut idx = 0;
        while idx < v.len() - 1 {
            if v[idx] < v[idx+1] {
                found = true;
                i = idx;
                j = idx + 1;
            }

            idx += 1;
        }

        if !found { return None }

        idx = j;
        while idx < v.len() {
            if v[idx] > v[i] {
                k = idx;
            }

            idx += 1;
        }

        let v_i = v[i];
        let v_k = v[k];
        //println!("v1 {}",v);
        //println!("changing i {} for k {}, {}->{}",i,k,v_i,v_k);
        *v.get_mut(i) = v_k;
        *v.get_mut(k) = v_i;
        //println!("v2 {}",v);

        // reverse everything after i
        let mut first = v.slice(0, i+1).to_vec();
        let mut last   = v.slice_from(i+1).to_vec();
        //println!("first last {} {}",first,last);
        last.reverse();
        //println!("last {}",last);
        first.extend(last.into_iter());
        //println!("new first {}",first);

        self.current = first.to_vec();
        Some(first)
    }
}

fn main() {
    let v = vec![0u,1,2,3,4,5,6,7,8,9];
    let mut l = Lexicographer::new(v);
    let n = 1000000;
    let mut idx = 0u;
    // println!("nth {} -> {}", n, l.nth(n).unwrap());

    for i in l {
        idx += 1;
        println!("{} {}",idx,i);

        if idx > n { break; }
    }
}
