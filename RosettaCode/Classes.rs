


fn main(){


    let x: usize = 5;

    let mut vec = vec![0; x];
    let i:usize;

    let mut v_copy ;

    for i in 0..x{
        vec[i] = i;
        println!("{}",vec[i]);
    }

    v_copy = create_new(&vec);


    let mut j : usize;
    j = 0;
    for i in 0..x/2{
        vec[j] = v_copy[i];
        vec[j+1] = v_copy[i+x/2];
        j += 2;
    }

    for i in 0..x{
        println!("{}",vec[i]);
    }

    
}


fn create_new<T: Clone>(vec: &[T]) -> Vec<T> {
    let mut newvec = vec.to_owned();
    newvec
}
