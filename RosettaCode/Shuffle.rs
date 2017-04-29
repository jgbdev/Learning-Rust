


fn main(){

    let input: [usize ; 7] = [8,24,52,100,1020,1024,10000];
    for item in input.iter() {
        println!("{} {}",item, shuffle(*item));
    }


}


fn shuffle(size : usize) -> i64 {

    let mut vec = vec![0; size];

    for i in 0..size{
        vec[i] = i;
    }

    let test = create_new(&vec);

    let mut count :i64;
    count = 0;

    while  {
        let v_copy = create_new(&vec);
        let mut j : usize;
        j = 0;
        for i in 0..size/2{
            vec[j] = v_copy[i];
            vec[j+1] = v_copy[i+size/2];
            j += 2;
        }
        count=count+1;
        !check_equal(&test, &vec)
    } {}
    count
}


fn create_new<T: Clone>(vec: &[T]) -> Vec<T> {
    let newvec = vec.to_owned();
    newvec
}

fn check_equal<T: PartialEq>(va : &[T], vb: &[T])->bool {
    va.iter().zip(vb).all(|(a,b)| a == b)
}
