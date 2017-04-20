fn main() {


    let v = vec![1,2,3];


    let v2 = v;
    //v2 now has ownership, we get an error if we access v;
    //println!("Error ! {:?}", v);
    //error: use of moved value: `v`
    //This is to prevent data race connditions

    //If v2 and v could still acess the underlying data
    //If we then modified v, e.g truncated it to length 2 this would make v2 Invalid
    //v2 would still think that the vector is of length 3

    //This also is applied to functions,
    //Arguments to function take ownernship

    //----------------------------
    //Copy types

    let vv = 1;
    let va = vv;

    println!("vv is {}", vv);

    //Because i32 (As do all primitive types) implement the copy function, the data is moved,
    //not just the point, like the above examples with Vectors
    //therefore there are no issues with race conditions and we are free to use the original var

    //---------------------------
    //Borrowing
    //Pass reference of the variable to your function
    //The functionn then "borrows" ownership
    //The reference is then immutable in the funcntion

    bar(&v2);
    println!("v after bar {:?}", v2);



    //&mut reference
    //&mut T allows mutation of the reference

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }

    println!("{}",x);

    //If we remove the scope brackets, it fails
    //Here are the following rules for borrinw in Rust
    //1. Any borrow must last for a scope no great than that of it's owner
    //2. Either
    //   a. One of more refrence (&T) to a resource
    //   b. Exactly one mutable reference (&mut T)

    //This is a similar definition to a data race, two pointers accessing memory at the same time
    //One of the pointers is writing and not sychronized


    //Thinking in scopes

    let mut base = 5;
    let y = &mut base;

    *y += 1;
    //println!(" base: {}", base);
    //Accessing base here returns an error, as y can mutate base
    //We need to end the scope of y, before we can access base


    let mut base2 = 5;
    {
        let yy = &mut base2;
        *yy += 1;

    }

    println!("base2 : {}", base2);


    //References
    //Use after free
    //If we try and access a variable declared in a score, we get an error
    //println!("{}", yy);


    let yy : &i32;
    let x = 5;
    //yy = &x;
    //println!("{}",yy);

    //If we decalare the reference, before the variable it refers to we get the same
    //error as above
    //This is because resources in the same scope are freed in the opposite order that they
    //were declared



    //---------------------------
    //Lifetimes
    //Dangling pointer issue when borrowed resouce is freed

    /*

    let r;
    {
        let i = 1;
        r = &i;
    }

    println!("{}",r);

    */

    //E.g r is still pointing to i, out of i's scope. Rust reports this error

    //Sometimes the compiler can't deduce if a reference will be valid outside a block

    let line = "lang:en=Hello World!";
    let lang = "en";


    let res;
    {
        let p = format!("lang:{}=",lang);
        res = skip_prefex(line, p.as_str());
    }
    println!("{}",res);

    //If skip_prefex block above would be invalid if skip_prefex didnn't have the Lifetime
    //params. Else the compiler would not know if the reference returned is still living when
    //calling println at the end of the block


    //Example 1.

    let (four,nine ) = (1,1);


    print_refs(&four, &nine);

failed_borrow();

    //Lifelines in Structs
    //struct PointRef<'a>
    //A lifetime that last as least as long as the struct



}
//a,b lifeline must be at least as long as print_refs
fn print_refs<'a , 'b>(x: &'a i32 , y: &'b i32){
    println!("x is {} y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>()  {
    let _x = 12;
    let y: &'a i32 = &_x;
    // ERROR: `_x` does not live long enough
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one.

    //If _x was to last longer than the function, e.g a parameter, than the function would
    //be valid. We could also remove 'a from y, which indicidates that y doesn't have to
    //live at least as long as failed borrow


}



struct PointRef<'a>{
    x : &'a mut i32,
    y : &'a mut i32,
}



fn skip_prefex<'a,'b>(line: &'a str, prefix: &'b str) -> &'a str {
    line
}




fn foo(v: &Vec<i32>){
    //v.push(5); We can't do this, v is immutable
}

fn bar(v: &Vec<i32>){
    println!("v in bar {:?}", v)
}


fn take(v: Vec<i32>){


}
