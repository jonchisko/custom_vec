use custom_vec::MyVec;

fn main() {
    let mut vec = MyVec::<usize>::new();
    vec.push(1usize);
    vec.push(2usize);
    vec.push(3usize);
    vec.push(4usize);
    vec.push(5usize);


    assert_eq!(vec.capacity(), 8);
    assert_eq!(vec.length(), 5);
}
