fn main() {
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin().read_line(&mut a).unwrap();
    std::io::stdin().read_line(&mut b).unwrap();
    let a = a.trim();
    let b = b.trim();
    let a = a.parse::<u64>().unwrap();
    let b = b.parse::<u64>().unwrap();
    println!("{}", a + b);
}