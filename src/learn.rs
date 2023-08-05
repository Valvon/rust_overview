/*
    Datentypen
*/
fn data_types(){
    // declared with let
    let int : i32 = 1;
    // types are inferred, don't need to annotate
    // mutability needs to be declared
    let mut int2 = 2_000_456; // _ for readability

    let float : f32 = 2.4;
    let boolean1: bool = true;
    let string : str = "oh yeah";
    let v: Vec<u8> = vec![1, 2, 3];

    // compiler type inference would now recognise int2 as u64
    int2 = int2 + 2u64;
    int2 += 1;
}



/*
Variablen und Operationen 
*/

fn operations() -> i32 { //  you can also have a return type declared
    let int1 = 45;
    let int2 = 32;

    // macro for printing
    print!("{}", int1 + int2);
    print!("{}", int1 - int2);
    print!("{}", int1 / int2);
    print!("{}", int1 * int2);
    print!("{}", int1 % int2);

    5 // returned value
}

fn shadowing_and_scope(){
    let a = 1;
    let mut b = 23;
    {
        let a = 42;
        println!("{}", a); // 42, original a is shadowed
        b *= 2
    }
    println!("{}", a); // 1
    println!("{}", b); // 46 because b wasn't shadowed and still in scope
}

fn ranges(){
    let range = 0..10; // from 0 to under 10 (9) , contains  start <= x < end
    let range2 = std::ops::Range {start: 3, end: 5}; // underlying commands
    let a = range.contains(&10); // false
    let b = range.contains(&9); // false

    // Slice, access index-wise with a range
    let name = String::from("jeremias");
    println!("{}", &name[0..1]); // "j"
    println!("{}", &name[0..3]); // "jer"

}

fn static_and_const(){
    // const, used for constant values not to be changed or reassigned, const mut is illegal
    const integervalue : i32= 25;
    // static, used for global values, can also be mutable
    static integervalue2 : i32 = 26;
    static mut integervalue3 : i32 = 26;
}

/*
    Ownership und Borrowing
*/
//       Normal Borrowing

// This will fail
fn takes_ownership(text : String){
    println!("{}",text);
}

fn not_possible(){
    let text : String =String::from("jeremias"); 
    takes_ownership(text);
    println!("{}",text); // this fails as text is invalidated 
}



// This will work
fn borrows(text : &String){
    println!("{}",text);
}

fn possible() {
    let text : String =String::from("jeremias"); 
    borrows(&text);
    println!("{}",text); // 45
}


//      Mutable Borrowing
// allows only one borrow at a time.

// this works
fn borrows_mutable(text : &mut String){
    println!("{}",text); // Jeremias
    *text = format!("{}{}", text, " 13")
}

fn possible() {
    let mut text = String::from("jeremias"); 
    borrows_mutable(&mut text); // this works

    println!("{}",text); // Jeremias 13
}


// this does not
fn not_possible() {
    let mut text = String::from("jeremias"); 
    borrows_mutable(&mut text); // this works
    let mut a = text; // this still works, because the function is finished with borrowing

    println!("{}",text); // this now doesn't work because text is mutably borrowed by a!
}

//      Copy and Clone Trait
// we can use these traits to automatically move/borrow
// u8 implements Copy, could be used to implement copy on own data structures
fn copy_trait(){
    let x: u8 = 123;
    let y = x;
    // x can still be used
    println!("x={}, y={}", x, y);
}

fn clone_trait(){
    // Vec<u8> implements Clone, but not Copy
    let v: Vec<u8> = vec![1, 2, 3];
    let w = v.clone();

    // using
    // let w = v 
    // would *move* the value, rendering v unusable.
    // println!("{}", v) would fail
}


/*
        Kontrollstrukturen und Schleifen

*/

fn if_else_decisons(){
    let n = 5;
    if 1 == n {
        n+= 2;
    } else if n < 6 {
        n -= 1;
    } else {
        n = n + 3;
    }
}

fn infinite_loop() {
    let mut counter = 0;
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue; // Skip the rest of this iteration
        }

        if count == 5 {
            println!("OK, that's enough");
            break; // Exit this loop
        }
    }
}


fn while_loop(){
    let mut n = 0;
    while n < 5 {
        println!("{}", n);
        n += 1;
    }
}

fn for_loop(){
    for i in 1..100{
        println!("{}", i) // prints until 99
    }

    for i in 1..=100{
        println!("{}", i) // prints until 100
    }

    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter(){   
        println!("{}", name); // "Bob Frank Ferris"
    }

    for name in names.into_iter(){ // this iterator consumes the collection. It will not be available after this iteration
        println!("{}", name); 
    }
    // println!("{}", names[0]);  will FAIL! because into_iter takes ownership


    for name in names.iter_mut(){ // this iterator allows mutable borrowing of each element
        *name = "ahahaha";
        println!("{}", name); 
    }
}

//      Match
fn match_statement(){
    for i in 0..10{
        match i {
            0 => println!("low"),
            2 | 4 | 6 => println!("low and even"), // 2 or 4 or 6
            7..=8 => println!("High {}", i), // range: 2..4 is not allowed here, has to be 2..=4 
            i if i > 8 => println!("Very High {}", i), // match Guard
            _ => println!("Anything else"), // catch else, needs to be included!
//          other => println!("Number {}", other)  // takes the missing case but also gives it an accessable variable name
        }
    }

    let boolean = true;
    // Match is an expression too, can assing values
    let binary = match boolean {
        false => 0, // 0 returned (to binary)
        true => 1, // 1 returned
    };
    println!("{}", binary); // 1


    let triple = (0, -2, 3);
   
    // Match TUPLE
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        (.., 2)  => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"), // `..` can be used to ignore the rest of the tuple
        _      => println!("It doesn't matter what they are"), // `_` means don't bind the value to a variable
        
    }

    // match ENUMS
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
    }
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
    }


    // match STRUCTS
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        // you can destructure structs and rename the variables,
        // the order is not important
    }
}

fn if_let(){
    let number = Some(7);
    let letter: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } 
    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!"); // Destructure will fail and print this
    }
}

fn while_let(){
   
        // Make `optional` of type `Option<i32>`
        let mut optional = Some(0);
    
        // This reads: "while `let` destructures `optional` into
        // `Some(i)`, evaluate the block (`{}`). Else `break`.
        while let Some(i) = optional { // aka as long as optional is equivalent to Some with any i as value.
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }
}


/*
        Structs, Enums
*/
fn this_struct(){
    // create own data types with Struct
    struct Fish {
        age : i32,
        species : String
    }

    // create Traits
    trait Swimming {
        fn swim(&self){
            println!("Swimming!");
        }
    } 

    // and implement them
    impl Swimming for Fish {
        fn swim(&self){
            println!("Swimming {}!", self.species);
        }  
    }
}

//      Enums
fn this_enum(){
    enum Store {
        Online,
        Local,
        Brand,
        PopUp
    }

    let store : Store = Online;
    match store {
        Store::Online => println!("Beep Boop"),
        Store::Local => println!("Cash please!"),
    }

    // enums may hold values
    enum Broker {
        Neobroker {name: String},
        BankDepot(String, i32),
    }
    let broker1 : Broker = Broker::Neobroker{name: String::from("Trade Republic")};
    let broker1 : Broker = Broker::BankDepot(String::from("Deutsche Bank"), 42);

    // we can also define methods for enums
    impl Broker {
        fn buyIn(){
            println!("Buy, Buy, Buy!")
        }

        fn sellOut(&self){
        match self {
            Broker::Neobroker{..} => println!("Sell!!!!"),
            _ => ()
            
        }
        }   
    }
    broker1.sellOut() // Sell!!
}


//      Option<T>()
    // enum Option<T> {
    //    None,
    //   Some(T),
    //}

fn options(){
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // We cannot add Some(value) with value
    let sum = some_number + 4; // this fails
}

fn realistic_options(option : Option<i32>) -> Option<i32> {
    match option {
        Some(i) => { // if the Option hold a value, return it incremented
            println!("The number is {}", i);
            Some(i+1)
        },
        None => Some(0), // If the Option holds no value, return 0
    }
}
/*
Pattern Matching
*/

// Ownership and Borrowing

/*
Collections
Iterators
Struct String
Generics and Traits
*/
/* 
Fehlerbehandlung
*/

//          Options
// can be None or Some(value) with value being the content 
fn checked_result() {

}

/*
Lifetime
Closures
Smart Pointers
OOp

*/
/*
Other Random Stuff we used
 */
use rand::Rng;

 fn random() {
    let mut rng = rand::thread_rng();
    let mut n = rng.gen::<u32>;
 }

 // Type name
 fn print_type_of<T>(identifier: &str, _: &T) {
    println!("The type of '{identifier}' is '{}'", std::any::type_name::<T>())
}

fn main() {
    let my_variable=1_0;
    print_type_of("my_variable", &my_variable);
}
 