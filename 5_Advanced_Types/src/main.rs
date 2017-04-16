
struct Point {
    x : i32,
    y:  i32,
}

fn main() {

    //Structs
    let origin = Point(x:0, y:0);
    let mut p = Point {x:0, y:0};

    p.x = 2;
    println!("New x {0}", p.x);

    //Rust doesn't allow mut in the struct definition
    //However you cna still but &mut pointers in Structs

    //Tuple Structs
    struct PointT(i32,i32,i32);
    let o = PointT(0,0,0);

    //Single length Tuple Struct
    struct Inches(i32);

    let length = Inches(32);


    //Enum
    //Type that represents data that is one of several possible variantns
    let x: Message = Message::Move { x:3 , x:4};
    enum BoardGameTurn {
        Move { squares: i32},
        Pass,
    }

    let y: BoardGameTurn = BoardGameTurn::Move { squares : 1}

    //Move is in different scope, so preventing conflicts
    //Value of an enum type contains information about what variant it is
    //Sometimes called a `tagged union` as data includes a `tag` indicating what type it is


    //Enum constructor can be used like a function
    let m = Message::Write("Hello, world".to_string());
    let f = foo("Hello, world".to_string());

    //foo is identical to the enum constructor


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
