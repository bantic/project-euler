fn long_divide(numerator: uint, divisor: uint, precision:uint) -> Vec<uint> {
    let mut result:Vec<uint> = vec![];

    let mut times:uint;
    let mut remainder:uint = numerator;


    while result.len() < precision {
        times = remainder*10 / divisor;
        //println!("times {}",times);
        result.push(times);
        remainder = remainder*10 - times*divisor;
        //println!("remainder {}",remainder);
    }

    result
}

fn is_repeat(candidate: Vec<uint>, list:Vec<uint>) -> bool {
    let c_len = candidate.len();
    let l_len = list.len();

    if l_len != c_len*2 {
        println!("l_len != c_len*2, false. {},{}",l_len,c_len);
        return false;
    } else {
        println!("l_len = c_len*2. {},{}",l_len,c_len);
    }

    let first = list.slice(0,c_len);
    let last  = list.slice_from(c_len);
    println!("first,last,candidate {},{},{}", first, last, candidate);

    first == candidate.as_slice() && last == candidate.as_slice()
}

fn find_cycle(list:Vec<uint>) -> uint {
    let mut max = 0u;
    let mut tmp:Vec<uint> = vec![];
    let mut cycle_candidates: Vec<Vec<uint>> = vec![];
    let mut last_candidate: Vec<uint> = vec![];

    let mut idx = 0u;
    while idx < list.len() - tmp.len() {
        tmp.push(list[idx]);

        //println!("making subs, idx+1:{},tmp.len():{}",idx+1,tmp.len());
        let subs = list.slice(idx+1, idx+1+tmp.len());

        //println!("comparing {} == {}",tmp,subs);

        if tmp.as_slice() == subs {
            println!("checking {} for repeat in {}",last_candidate,tmp);
            if is_repeat(last_candidate.to_vec(), tmp.to_vec()) {
                println!("skipping repeat: {},{}",tmp,last_candidate);
            } else {
                last_candidate = tmp.to_vec();
            }
        }

        idx += 1;
    }

    last_candidate.len()
}

fn main() {
    let precision = 100u;
    let num = 11u;
    let result = long_divide(1u,num,precision);
    println!("1/{} {}",num,result);
    if is_repeat( vec![0u,9u], vec![0u,9u,0u,9u] ) {
        println!("is repeat");
    } else {
        println!("not repeat");
    }
    //println!("MAX: {}",find_cycle(result));
    //for i in range(2u,100u) {
        //println!("1/{} {}",i,long_divide(1u,i,precision));
    //}
}
