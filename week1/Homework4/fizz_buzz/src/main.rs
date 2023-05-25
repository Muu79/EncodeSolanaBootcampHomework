fn main() {
    let mut count = 0;
    for i in 0..=301{
        if i % 5 == 0 && i % 3 == 0{
            println!("fizz buzz");
            count+=1;
        }
        else if i % 3 == 0{
            println!("fizz");
        }else if i % 5 == 0{
            println!("buzz");
        }
    }
    println!("\"fizz buzz\" was printed {} times", count)
}
