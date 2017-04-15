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
    let va = v;

    println!("v is {}", v);

    //Because i32 (As do all primitive types) implement the copy function, the data is moved,
    //not just the point, like the above examples with Vectors
    //therefore there are no issues with race conditions and we are free to use the original var




}


fn take(v: Vec<i32>){


}
