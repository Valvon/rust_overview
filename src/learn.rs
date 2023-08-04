/*
    Datentypen
*/
fn data_types(){
    // declared with let
    let int : i32 = 1;
    // types are inferred, don't need to annotate
    // mutability needs to be declared
    let mut int2 = 2;

    let float : f32 = 2.4;
    let boolean1: bool = true;
    let string : str = "oh yeah";
    let v: Vec<u8> = vec![1, 2, 3];

    // compiler type inference would now recognise int2 as u64
    int2 = int2 + 2u64;
    int2 += 1;
}

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

/*
Variablen und Operationen 
*/
fn operations(){
    let int1 = 45;
    let int2 = 32;

    // macro for printing
    print!("{}", int1 + int2);
    print!("{}", int1 - int2);
    print!("{}", int1 / int2);
    print!("{}", int1 * int2);
    print!("{}", int1 % int2);
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
Structs, Enums
Pattern Matching
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