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

    //function to do the common fizz buzz problem involving modulo 5 and 3, but print the amount of times fizz buzz was printed and print fizz and buzz if the number is divisible by 3 or 5 repsectively
    fn fizz_buzz(){
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
    
}
