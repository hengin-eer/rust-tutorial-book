fn main() {
    for x in 1..10 {
        let s = (1..10)
            .map(|y| format!("{:3}", x * y))
            .collect::<Vec<String>>().join(",");
        println!("{}", s);
    }
}