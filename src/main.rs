fn main() {
    let vetor = vec![1,2,3];
    let mut my_iterator = vetor.iter();
    let x = my_iterator.next();
    println!("Value = {:?}", x);

    let x = my_iterator.next();
    println!("Value = {:?}", x);

    let x = my_iterator.next();
    println!("Value = {:?}", x);

    let x = my_iterator.next();
    println!("Value = {:?}", x);
}
