fn main(){
    let number = 6;

    if number % 4 == 0 {// if alwasy intitializes booleand data type
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else { 
        println!("number is not divisible by 4, 3, or 2");
    }
    // this line wont work as in this first type is 5 and the second is six
    // let number = if condition { 5 } else { "six" };
    // .................................................Repetition with Loops....................................
         loop{//syntax for creating loop
            println!("hello");
            break;
         }
         let mut counter = 0;
        let result = loop{
            counter+=1;
             
            if counter == 10{
                break counter * 2;
            }
        };
        println!("{result}");

_main();
__main();
___main();
}
fn _main() {
    let mut count = 0;
   // naming a loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
//.......................................loop with while.......................................
fn __main(){
    let mut number = 3;
    while number != 0{
        println!("{number}!");
        number-=1;
    }
    println!("{number}!");
}
//.......................................loop with for.......................................
fn ___main(){
    let a = [10,20,30,40,50];
    
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}