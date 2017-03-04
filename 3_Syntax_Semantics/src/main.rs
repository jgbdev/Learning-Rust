fn main() {

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

    println!("Finished!");
}


fn print_number(x: i32){
        println!("x is {}",x)
}

fn print_sum(x: i32, y: i32){
        println!("x+y is {}",y+x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}
