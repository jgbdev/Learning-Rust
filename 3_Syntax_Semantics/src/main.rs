fn main() {

    //----------------------------
    //Variable bindings
    //----------------------------
    let x: i32 = 5;
    {
        let y: i32 = 3;
        println!("({},{})",x,y)
    }

    //Invalid, y is out of scope
    //println!("({},{})",x,y)

    let y: i32 = 10;
    print_number(x);

    print_sum(x,y);

    println!("x+1={}",add_one(x));

    //Expressions and Statements

    //Invalid: let x = (let y = 5);
    //TODO: Flesh out

    // Divergence

    //diverges()  Calling this will not return anything, due to ! ret type
    //Similar to how println! doesn't return anything

    //We can assign a divergence to any type
    //let x: i32 = diverges();
    //let x: String = diverges();
    //----------------------------
    // Function Pointers
    //----------------------------

    // We can create variable bindings which point to functions

    // Without type inference:
    let f: fn(i32) -> i32 = add_one;

    // With type inference:
    let f = add_one;

    let six = f(5);

    //----------------------------
    // Primitive Types
    //----------------------------

    //Standard fixed sized types, signed/unsigned
    //Variable sixed type depend on underlying OS

    //Arrays
    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]

    //Initiliasing arrays with the same value
    let a = [0; 20]; // a: [i32; 20]Run

    //Subscript notation to access array values

    let b = a[2];


    //Slicing
    //Slice refers to a view inside an arrays
    //Allow safe access without copying


    let a = [0,1,2,3,4,5,6];
    let complete = &a[..]; //All elements
    let middle = &a[1..4]; //Only elements from index 1 to 4


    //Tuples
    //Ordered list of fixed sized

    let x = (1,"hello");
    let x: (i32, &str) = (1, "Hello");

    //&str should be read as a "string slice"

    //You can assign a tuple into annother if they have the same type and arrity (lenght)

    let mut x = (1,2);
    let y = (2,3);

    x = y;

    let (x,y,z) = (1,2,3);
    //This overwrites the bindings in the previous section

    //Tuple indexing

    let tuple = (x,y,z);

    println!("{} , {} , {}" , tuple.0, tuple.1 , tuple.2);



    //Vectors
    //Growable/Dynamic arrays

    let mut v = vec![1,2,3,4,5];


    //Index with usize type
    let i: usize = 0;
    println!("Hi! {}", v[i]);

    //Add an item
    println!("Old vector, v: {:?}" , v);
    v.push(123);
    println!("New vector, v: {:?}" , v);





//





    println!("Finished!");
}


fn diverges() -> ! {
    panic!("This function never returns!");
}

//fn early_return(x: i32) -> i32 {
    //return x + 1; // Return works

    //let y: i32 = 10; Becomes unreachable
//}



fn print_number(x: i32){
        println!("x is {}",x)
}

fn print_sum(x: i32, y: i32){
        println!("x+y is {}",y+x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}
