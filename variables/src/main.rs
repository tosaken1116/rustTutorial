fn main() {
    println!("Hello world!");

    // another_function(5);
    print_labeled_measurement(5,'h');
    let y:i32 = {
        let x:i32 = 3;
        x+1
    };
    println!("The value of y is: {}", y);
    let x:i32 = five();
    println!("The value of x is: {}", x);

    let z:i32 = plus_one(5);
    println!("The value of z is: {}", z);
    let mut count:i32 = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining:i32 = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

// fn another_function(x:i32){
//     println!("The value of x is:{}",x);
// }

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32{
    5
}

fn plus_one(x:i32)-> i32{
    x+1 //plus 1
}