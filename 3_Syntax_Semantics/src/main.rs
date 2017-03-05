fn main() {


    //Variable bindings

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
    let x: i32 = diverges();
    let x: String = diverges();

    // Function Pointers

    // We can create variable bindings which point to functions

    // Without type inference:
    let f: fn(i32) -> i32 = plus_one;

    // With type inference:
    let f = plus_one;

    let six = f(5);


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
