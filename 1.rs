fn main() {
    let mut sum = 0;
    for number in 1..1000 {
        if number % 5 == 0 || number % 3 == 0 {
            sum += number;
        }
    }
    println!("{}", sum)
}
