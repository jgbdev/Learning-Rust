
struct Point {
    x: i32,
    y:  i32,
}


impl Point {

    fn Origin() -> Point {
        Point {x: 0, y:0}
    }

    fn new (x:i32, y:i32) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {

    p1 : Point,
    p2 : Point,
}

trait HasArea {
    fn area (&self)-> i32;
}


impl Rectangle {
    fn area(&self)-> i32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1-x2) * (y1-y2)).abs()
    }
}


struct PointRef<'a>{
    x : &'a mut i32,
    y : &'a mut i32,
}


fn print_refs<'a , 'b>(x: &'a i32 , y: &'b i32{
    println!("x is {} y is {}", x, y);
})

fn main() {

    //Structs
    let origin = Point {x:0, y:0};
    let mut p = Point {x:0, y:0};

    p.x = 2;
    println!("New x {0}", p.x);

    let rect = Rectangle {
        p1 : Point::Origin(),
        p2 : Point::new(3,4),
    };

    println!("Area {}", rect.area());



    //Rust doesn't allow mut in the struct definition
    //However you cna still but &mut pointers in Structs

    //Tuple Structs
    struct PointT(i32,i32,i32);
    let o = PointT(0,0,0);

    //Single length Tuple Struct
    struct Inches(i32);

    let length = Inches(32);


    //We use the impl block to define methods linked to structures


    //Enum
    //Type that represents data that is one of several possible variantns
    let x: Message = Message::Move { x:3 , y:4};
    enum BoardGameTurn {
        Move { squares: i32},
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares : 1};


    //Move is in different scope, so preventing conflicts
    //Value of an enum type contains information about what variant it is
    //Sometimes called a `tagged union` as data includes a `tag` indicating what type it is


    //Enum constructor can be used like a function
    let m = Message::Write("Hello, world".to_string());
    let f = foo("Hello, world".to_string());

    //foo is identical to the enum constructor


    //use
    //use can reference nested types in an enum
    use pass = BoardGameTurn::Pass;


    //Traits
    //Define rules about a functionality a type must provide
    //Defines method signatures, which must be implemented

    //E.g Rectangle has an area trait.
    fn print_area<T>(shape: T)){
        println!("This shape has an area of {}", shape.area());
    }
}


fn foo(x: String) -> Message {
    Message::Write(x)
}


enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y : i32},
    Write(String), //Here write is a tuple like struct
}
