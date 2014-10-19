fn next_triangle_vec(vec: Vec<uint>) -> Vec<uint> {
    let mut next_vec = vec![1];

    for i in range(0, vec.len() - 1) {
        next_vec.push( *vec.get(i) + *vec.get(i+1) );
    }

    next_vec.push(1);
    next_vec
}

fn triangle_vec(num:uint) -> Vec<uint> {
    let mut vec = vec![1,1];
    let mut cur_num = 1;
    while cur_num < num {
        cur_num = cur_num + 1;
        vec = next_triangle_vec(vec);
    }
    vec
}

fn main() {
    let mut max = 0;
    let tri = triangle_vec(20*2);
    println!("TRI {}",tri);
    for &i in tri.iter() {
        if i > max { max = i; }
    }
    println!("max: {}",max);
}
