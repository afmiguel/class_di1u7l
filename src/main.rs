fn main() {
    let vetor = vec![1,2,3,4,5,6];
    let mut x =
        vetor
        .iter()
        .filter(|v|
            {
                println!("{v}");
                *v % 2 == 0
            }
        );
    println!("Vamos iniciar a iteração...");
    println!(">>> {:?}", x.next());
    println!("{:?}", x.next());
    println!("{:?}", x.next());
    println!("{:?}", x.next());
}

