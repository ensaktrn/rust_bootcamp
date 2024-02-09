fn main() {
   

    //Boolean
    let _is_rust_fun: bool = true;

    //Integers
    //16,32(default),64,128,
    let x =42;
    let _y: i128 = 42;

    //floating point number
    //32,64(default)
    let _f: f64 = 3.1;

    //Characters
    let _letter_a: char = 'a';

    //Strings
    let _message = "Hello World!";
    let _my_string = String:: from("Hello World!");

    //Arrays
    let _days_of_week: [&str;7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday"
    ];
    let _first_element_of_array = _days_of_week[0];
    let _last_element_of_array = _days_of_week[_days_of_week.len() - 1];

    //Slices
    let _slice = &_days_of_week[1..3];
    let _first_element_of_slice = _slice[0];

    //Tuples
    let person = ("Enes",22);

    let _name = person.0;
    let _age = person.1;

    //Unit Type
    let result = add(x,12);
    println!("The result is {}",result);

    //Variables
    let mut _number = 15;
    _number = 18;

    // if-else statement
    let x = 42;
    if x < 0 {
        println!("x is negative");
    }else {
        println!("x is not negative");
    }

    //while loop
    let mut i = 1;

    while i < 5 {
        println!("{}",i);
        i+=1;
    }

    //for loop
    for number in 1..5 {        // if you want to include 5 make 1..=5
        println!("Number is {}",number);
    }

    //loop
    let mut counter = 0;
    loop {
        println!("Counter is {}",counter);
        counter += 1;
        if counter == 6{ 
            break;}
    }

    //match(switch)
    let num = 5;  
    match num {
        1 => println!("The number is one!"),
        2 => println!("The number is two!"),
        3 => println!("The number is three!"),
        _ => println!("The number is something else!"),
    }
    let result = match num {
        1 => "The number is one!",
        2 => "The number is two!",
        3 => "The number is three!",
        _ => "The number is something else!",
    };
    println!("{}", result);

    let my_string = String::from("hello World");
    print_string(&my_string);
    println!("{}",my_string);

} 

fn add(x: i32, y: i32) -> i32{
    x+y
}

fn print_string(s: &String){
    println!("{}", s);
}