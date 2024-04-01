fn main() {
    let mut vec0 = vec![22, 44, 66];

    fill_vec(&mut vec0);

    assert_eq!(vec0, vec![22, 44, 66, 2]);
}

fn fill_vec(vec: &mut Vec<i32>) {
    // let vec = vec;

    vec.push(88);
}
