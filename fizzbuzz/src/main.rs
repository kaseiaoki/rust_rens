fn main() {
   for i in 1..300 {
     fizzbuzz(i);
    }
}

fn fizzbuzz(n: i32){
   match n{
        n if n%5==0 && n%3==0 => println!("fizzbuzz"),
        n if n%3==0 => println!("fizz"),
        n if n%5==0 => println!("buzz"),
        _ => println!("{}", n),
    }
}
