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
    // checked arithmetic returns Some or None (later)
    let a = int1.checked_mul(int2); 
    let a = int1.checked_add(int2);
    let a = int1.checked_div(int2);  
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

// Also interesting for borrowing: 
// Some objects need longer lifetime (statics), 
// we can allocate memory on the heap (not stack)
// which will be freed after the owner is dropped.

/*
    Ownership und Borrowing
*/
// Ownership is saved on the stack
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

fn overlapping_mut_borrowing(){
    let cnt = &mut new_counter("my counter");
    increment(cnt);
    increment(cnt);
    let cnt_ref1 = &(*cnt); 
    let cnt_ref2 = &(*cnt); 
    as_string(cnt_ref1);
    increment(cnt); // this fails, because cnt_ref2 is still borrowing cnt
    as_string(cnt_ref2) // this is because cnt_ref2 is borrowed until it's last use HERE
    //if the last line was removed, it would compile, or increment was under the alst use of cnt_ref2
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

// Copy:
// we can use this traits to automatically create a new owner to the data
// owner holds data
// data gets copied
// owner1 and owner2 hold same memory address 
// u8 implements Copy. Copy can be implemented on own data structures
fn copy_trait(){
    let x: u8 = 123;
    let y = x;
    // x can still be used
    println!("x={}, y={}", x, y);
}

// Structs are moved because they don't have the Copy Trait
struct RandomStruct{}
fn random_func(mut x :  RandomStruct) -> RandomStruct{ x }
fn copy_fails(){
    let mut a = RandomStruct{};
    random_func(a); // this is fine, but a is not the owner anymore
    random_func(a); // this is not fine (panic), because a has no ownership rights
} 
// this would work if RandomStruct had Copy Trait, as random_func would get a as a copy
// Beware of Dangling Pointers! If a was to not live longer than the functions, then:
// owner dropped -> reference to data dropped -> data droppped (even though random_func() uses it)

// Clone
// once cloned, we have completely new owners with their own data, in no way depend on each other
fn clone_trait(){
    // Vec<u8> implements Clone, but not Copy
    let v: Vec<u8> = vec![1, 2, 3];
    let w = v.clone();
    use_v(v); // possible
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
//          other => println!("Number {}", other)  // takes the missing case but also binds it to accessable variable name // Binding
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
pub struct GnStruct(String); // Tuple Struct
fn main(){
    let a = GnStruct("lol".to_string());
    println!("{}",a.0); // access tuple parameter
    
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
    let a = some_char.take(); // now a is owner of Some('e') and some_char is None
    let absent_number: Option<i32> = None;

    some_number.is_none(); // false
    some_number.is_some(); // true

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

fn really_short_option(option : Option<i32>) -> Option<i32>{
    let a = option?; // this returns None instantly, if the Opion is not of kind Some(i)
    a + 1 //if it was Some(i) a is the value i and we can work with it
}

// Result is also an enum, but treated in section about Errors

/*
Pattern Matching
*/

fn pattern_matching(){
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // from now on, a and b are accessible as the values of point p
    // matching the structure of Point with variables to an instance


    // we've mostly handled this already, but there are some edge cases
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // This Statement shadows the outer "y", it will not use y as value 10
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x); // But this will print x= 5 and y = 10, as the scope of y (in match statement) ended
}

// Ownership and Borrowing was already explained, see above
// Guards, Ranges and Bindings were also explained above

//      Decompose Types ???
// when will data be dropped?
fn decomposing(){
    enum Broker {
        Neobroker(String),
        BankDepot(String, i32),
    }
     // Neobroker Dropped once the condition has been evaluated
     let a = Broker::Neobroker("If condition".to_string());
     if Broker::Neobroker("If condition".to_string()) == a { // btw for == you would need to implement PartialEq
         // Neobroker Dropped at the end of the block
        Broker::Neobroker("If body".to_string());
     }
    match Broker::Neobroker("Dropped at end of match") {
        Broker::Neobroker(b) if b == "Nothing" => (), // drop
        _ => () // drop
    }

    loop {
        // Tuple expression doesn't finish evaluating so operands drop in reverse order
        (
            Neobroker("Outer tuple first"),
            Neobroker("Outer tuple second"),
            (
                Neobroker("Inner tuple first"),
                Neobroker("Inner tuple second"),
                break, // now drop in reverse
            ),
            Neobroker("Never created"),
        );
    }

    
    {   // SPECIAL case
        let x = &mut 0;
        // Usually a temporary would be dropped by now, but the temporary for `0` lives
        // to the end of the block.
        println!("{}", x);
    }

    // The temporary that stores the result of `temp()` only lives until the
    // end of the let statement in these cases.
    let x = Some(&temp());         // ERROR
    let x = (&temp()).use_temp();  // ERROR
}


//      Definition Refutability
fn refutability() {
    let some_value = None;
    let Some(x) = some_value; // this gives an error, as the Patterns don't match
    let (x,y) = (1,2,3); // also doesn't match

    // handle these Problems
    if let Some(x) = some_value { // if patterns match, print the value, else, do nothing
        println!("{}", x);
    }

    // example of how to handle possibly unmatching types
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    fn main() {
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

}
/*
Collections
*/
// stack, appendable array, resizable array
 fn vectors() {
    let mut vector = vec![1,2,3,4,5,9];
    let vector2 = vec![80,35,658];
    vector.extend(vector2); // append to it

    let vector3 : Vec<String> = Vec::new();

 }

 use std::collections::HashMap;
fn hash_map() {
    let mut reviews = HashMap::<String, String>::new(); // type can also be inferred
    reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );  
    reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    reviews.len(); // length

    reviews.remove("Pride and Prejudice"); // removes Element with key "Pri.."
    reviews.contains_key("Pride and Prejudice"); // now false, was true before

    let to_find  = ["Pride and Prejudice", "Adventures of Huckleberry Finn"];
    for &book in &to_find { // borrow because else it would be consumed, in this case with &str that has copy trait it is not necessary
        match reviews.get(book) {
            Some(review) => println!("{book}: {review}"),
            None => println!("{book} is unreviewed.")
        }
    }

    let key = reviews.entry("Masterpiece"); // gets all keys to the entry
}
/*
Iterators
*/
fn iterators(){
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter(){   
        println!("{}", name); // "Bob Frank Ferris"
    }

    for name in names.into_iter(){ // this iterator consumes the collection. It will not be available after this iteration
        println!("{}", name); 
    }
    // println!("{}", names[0]);  will FAIL! because into_iter takes ownership


    for name in names.iter_mut(){ // this iterator allows mutable borrowing of each element
        *name = "ahahaha"; // value can be changed
        println!("{}", name); 
    }
    // iterators can be used very comfortably with map/filter and collect
    // filter() will keep the value if the closure inside evaluates to true
    let names1 : Vec<&str> = names.iter().filter(
        |name| match name.chars().next() {
            Some(i) => !(i == 'B'),
            None => true,
    }).cloned().collect(); // we need to clone, because filter returns an iterator over references to elements of the original iterator, so we receive &&str not &str


    // map() -> work with values and return them (potentially modified) as a copied iterator
    // collect() the whole iterator
    let mut names2 : Vec<String> = names1.iter().map(|name| name.to_string()).collect();
    // There also are
    // map_or("defaul value", |item| item_mapped)
    // map_or_else(|else| else_func(), |foo| foo_mapped)

    for name in names2.iter_mut(){ // this iterator allows mutable borrowing of each element
        let other = "no way its you, ";
        *name = format!("{}{}", other, name); // format! returns a String not str (hence the String::from in the vector declaration)
        println!("{}", name); 
    }

    names2.iter_mut().map(|name| name + "l"); // iterators are lazy and do nothing. use last() to consume the iterator 
    //btw this example is not full compilable because I am to dumb for working with Strings
    // counting entries with fold()
    names2.iter().fold(0, |count, _| count + 1);
    //also relevant
    //iter().max()
    //iter.min()

}

/*
Struct String
*/
fn string_basics(){
    // A String is basically a Vec<T>
    let mut s = String::new();
    s = "something".to_string();
    let a = String::from("Hello");

    //append
    s.push_str(" Yo");
    s.push('u');
    println!("s is {s}");

    // concatenation
    // let s3 = s + &a; // s is consumed
    let s3 = format!("{s}-{a}"); // much nicer, takes references and doesnt consume
    println!("{s3}");

    // don't acces with [0] but with a range to create a slice
    let b = &s3[0..4];
    println!("{b}"); // "some"

    let c = b.bytes();
    println("{c}");
}

/*
Generics and Traits
*/

//      Polymorphism
//      dyn-Keyword 
// Dynamic dispatch, have a varaible that has a specific type but declare it with dyn and an upper trait
// static dispatch is faster btw
fn log<T: Any + Debug>(value: &T) {
    let value_any = value as &dyn Any;

    match value_any.downcast_ref::<String> {
        Some(text) => println!("It's a string"),
        None => println!("No String"),
    }
}

// also possible with
fn log_d(value_any: &dyn Any) -> &dyn Any {
    value_any
}

//      Generic Type
fn generic_highest(list : &[T]) -> &T{ // This won't compile, just to show concept
    let mut highest = &list[0];

    for item in list {
        if highest < item { // compare the Types (they need to implement the comparison)
            highest = item;
        }
    }
    highest
}
//      Traits
pub trait Summary { // basically an interface with possible default behaviour
    fn summarize(&self) -> String { // default implementation of the method
        String::from("Read more here!");        
    }
}

pub struct Newspaper{
    pub headline : String,
    pub story : String,
}

impl Summary for Newspaper{
    fn summarize(&self) -> String { // implement a method from the trait specifically
        format!("{}", self.headline[])
    }
}

        // Sub-traits, inheritance
// if  a type implements Summary it must implement Display
trait Summary: Display {}

// as in Conditional Trait Implementation, you can use that for Inheritance
// start a hierarchy where all types that impl C have Summary and Display...etc.
impl <T: Summary + Display> C for T{

}

        // Trait Bounds
// this function needs item to implement Summary
fn needs_trait_short(item : &impl Summary) {
    println!("{}", item.summarize());
}

// useful if many items use this trait
fn needs_trait<T: Summary>(item : &T, item2: &T) {
    println!("{}", item.summarize());
}

// multiple traits
fn needs_two_traits(item: &(impl Summary + Display)) {
}
fn needs_two_traits_too<T: Summary + Display>(item: &T) {
}

// can be more clear with: 
fn needs_two_traits_clear<T, Uy>(item: &T, item2 : &U) -> i32
where // <T: Summary + Display, U: Summary + Clone>)
    T: Summary + Display,
    U: Summary + Clone,
    { 
        return 5;
}

// we of course can declare those in structs too!
pub struct KeyLocker <T: Debug + PartialEq> {
    name : T,
}

// returned Elements may implement a trait 
// WATCH OUT, you can only return a single type here, an if else with different types that implement the trait is not allowed
fn returns_summarizable() -> impl Summary {
    Newspaper{
        headline : String::from("You won't believe it"),
        story : String::from("Omg I like the game so much I have to..")
    }
}
        // Conditional Implementation

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
 // methods defined within this impl block will be available for any instance of Pair where the type parameter T implements both the Display and PartialOrd traits.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// impl ToString for any type that implements Display trait
 impl<T: Display> ToString for T {
}       
        // Copy,Clone,Display,Debug
// COPY trait is for types whose values can be duplicated simply by copying bits.
// is used by assignments like let x = y; and not explicitly
struct Foo {}   
impl Copy for Foo{}
// or
#[derive(Copy)]
struct Bar{}

// CLONE is type specific copy
// specifies .clone()
#[derive(Copy, Clone)]
struct FooBar{}

// Types that have Copy, must implement Clone
// your own structs can't implement Copy, when they have items that don't implement Copy Trait

// DISPLAY Trait 
// will also automatically implement the ToString trait, allowing .to_string
use std::fmt;

impl fmt::Display for FooBar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// DEREF
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0 // rust translate *y to *(y.deref)
    }
}

/* 
        Fehlerbehandlung 
*/


// Recoverable / Unrecoverable
fn unrecoverable() {
    let a = vec![1,5,2,7,3,8,3];
    let b = a[16]; // of course this panics and we can't handle it
}
fn recoverable(){
    let file = File::open("gfg.txt");
    println!("{:?}",file); // this will give a printed error of the Result<> that file has
    // we could prevent it with a match to Ok() and Err()
}
//Backtrace wofür?
// to find the origin of a panic

// the macro: panic!
// will stop the program and clear the stack
fn panic_hook(){ // setting a custom panic action
    panic::set_hook(Box::new(|x| {
        println!("Custom panic hook {x:?}");
        ExitCode::from(42).exit_process();
    }));
}
// Standard-Types from std-Lib (not sure what is meant, but)
/*  growable Strings like: "hello world"
    growable vectors: [1, 2, 3]
    optional types: Option<i32>
    error handling types: Result<i32, i32>
    heap allocated pointers: Box<i32> */


// unwrap and expect, when is it used?
//      
// Especially with Result<T,E>
    // T and E are generics. T can contain any type of value, E can be any error.
    //enum Result<T, E> { 
    //    Ok(T),
    //   Err(E),
    //}

    fn result_enum(){
        let success  : Result<i32, i32>= Ok(45); // types need to be defined
        let error : Result<i32, i32> = Err(0);
    
        success.is_ok(); // true
        success.is_err(); // false
        
        // also for Options!!
        // not good to use, at least those that panic and have no alternate behaviour
        let value = success.unwrap();  // panics if success is Err() or None
        let value2 = success.expect("Error message"); // same as unwrap with a message
        let value3 = success.expect_err("OK error message"); // panics if success is Ok() or Some()
    
        // Better use these
        let edge_case = 5;
        let value4 = success.unwrap_or(0);  // if there is an error, use the given value instead of panicking
        let value5 = success.unwrap_or_default(); // if there is an error, use the default value of the data type
        let value6 = success.unwrap_or_else(|| edge_case); // if there is an error, evalutate the Closure and return that value
    }
    
    fn result_example(i : Result<i32, i32>) -> Result<i32, i32> {
        match i {
            Ok(i) => {
                println!("{}", i);
                Ok(i*2)
            }
            Err(..) => {
                println!("Error");
                Err(1)
            }
        }
    } // Option<T> is also there for error handling


/*
        Lifetime 
        you need to specify lifetime parameters for functions or structs that use references
        &i32        // a reference
        &'a i32     // a reference with an explicit lifetime
        &'a mut i32 // a mutable reference with an explicit lifetime
        Lifetime Elision Rules (automatic lifetime from compiler)
        [1] for each parameter the compiler adds one lifetime parameter
        [2] only one parameter -> return value has lifetime of that parameter
        [3] if there is self as input parameter, the output value has lifetime of self
*/
//static has global lifetime, here, the counter lives forever
fn new_counter(name: String) -> &'static mut Counter {
    &mut Counter { name: name, counter: 0 }
}
// anonymous lifetime TODO

// this code fails without lifetime annotation, because at compile time we don't know if x or y is returned and so the borrow checker fails to know when which borrow ends
// adding it fixes the problem
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// result is dropped after longest() and is unassociated with the lifetime 'a, this doesn't work
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str() // this will fail!
}


/*
Closures
*/
// Closures are functions saved in a variable or passed to a function
// comparison
fn  add_one_v1(x: u32) -> u32 {return  x + 1;
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
}

// example usage 
fn example_closure(){
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// pass to a function
fn pass_me_a_func<F>(function : F) where F : Fn(i32) -> i32{
    let a = function(5);
    println!("{}", a); // prints 25
}

fn passing_a_func(){
    let func = |i  : i32| -> i32 { i*i};
    pass_me_a_func(func);
}

//also possible use
fn sort(){
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
}
// another example is found at iterators with filter() and map()


/*
Smart Pointers
*/
// Ownership and Borrowing ? Some things change.
// Smart pointers are pointers but also have additional metadata and capabilities

// Recursive Types with Box
// Box<T> for allocating values on the heap
enum List { 
    Cons(i32, List), // every Element contains a value and the next part of the List
    Nil, // the end of the list
}

fn recursive_list(){
     // Box is basically a pointer to a sized Object
    // hence the Box offers "indirection", so the compiler knows about the size of the list, value is now allocated on heap!
    let list = List::Cons(32, Box::new(List::Cons(12, Box::new(List::Nil))));
    let a = 45;
    let b = Box::new(a);
    println!("{}", *b);
}

// Reference Counting
//Rc<T> a reference counting type that enables multiple ownership
// It counts the amount of owners of a value
// creating a new owner is done by cloning the reference
use std::rc::Rc;
enum List { 
    Cons(i32, Rc<List>), // every Element contains a value and the next part of the List
    Nil, // the end of the list
}

fn reference_counting(){
    let a = Rc::new(Cons(100, Rc::new(Cons(10, Rc::new(Nil))))); // Rc with 100 has reference count of 1
    let b = Cons(3, Rc::clone(&a));  // Rc with 100 has reference count of 2, because it was cloned
    let c = Cons(4, Rc::clone(&a)); // Rc with 100 has reference count of 3, because it was cloned again
}

// Cell and RefCell 
// RefCell<T> enforcing borrowing rules at runtime
// needed for Interior Mutability, can mutate data even when there are immutable references
// when mutably borrowed, no more borrowers are allowed
use std::cell::RefCell;
fn refcell() {
    let c = RefCell::new(5);
    {
        let mut v=c.borrow_mut();
        *v +=1;
    }
    println!("{c:?}"); // 6
}

// RefCell and Rc can be combined
#[derive(Debug)]
enum List {
    Cons(i32, Rc<RefCell<List>>), // a reference counting Rc contains the refcell which is holding the next list enum
    Nil,
}

fn combined_rc_refcell(){
    let list = Rc::new(RefCell::new(Cons(5,Rc::new(RefCell::new(Cons(10, Rc::new(Nil)))))));
    if let Cons(ref mut v, ref mut _r) = list.borrow_mut() {
        *v +=3;
    }
}
// Explanation by Bing Chat
/*The borrow_mut method is called on the RefCell that wraps the list variable. 
This returns a mutable reference to the value inside the RefCell, which is the first element of the list.
The * operator is used to dereference this mutable reference, giving us access to the first element of the list itself.
The if let expression is used to pattern match on this first element of the list. 
The pattern used is Cons(ref mut v, ref mut _r), which matches against the Cons variant of the List enum.
If the first element of the list is an instance of the Cons variant, then the values inside this variant are extracted and bound to the variables v and _r. 
The ref mut keywords are used to indicate that these variables should be mutable references to the values inside the variant, rather than copies of those values.
Since v is a mutable reference to the first value inside the Cons variant, we can use it to modify this value directly. 
In this case, we add 3 to this value using the += operator.
If the first element of the list is not an instance of the Cons variant, then nothing happens and control flow continues after the end of the if let expression. */


/*
OOP TODO
*/

// Rust is not strictly an OOP language.
// Encapsulation: Structs
//  invariants with predefined methods to access data
//  Modules
mod abstract_module{
    pub trait Abstract{
        // ...
    }
}
// Inheritance: Traits
// Polymorphism: via Dynamic Dispatch and Trait Objects



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
 