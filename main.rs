fn main() {
    let mut even_numbers: Vec<i32> = Vec::new(); 
    let limit = 300;

    for num in 0..=limit {
        if num % 2 == 0 {
            println!("{}", num); 
            even_numbers.push(num); 
        }
    }

    println!("Even numbers stored: {:?}", even_numbers); 
}
