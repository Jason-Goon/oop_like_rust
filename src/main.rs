struct Point {
    x: f32,
    y: f32,
}
// both are just data structures or 'custom data types' 
struct Rect {
    height: u32,  // in rust we always specify types, some are inferred eq let x = 5 
    widht: u32,   // without specifying is inferred i32 but is recommended to spcify like let x: u8 = 5
}


//Rect has methods so it behaves like an object 
impl Rect {
    fn area(&self) -> u32 { // &self shortahnd for self: &Self means immutable borrow of the struct
        self.widht * self.height   
    } // everything is immutable by default & means borrow or a reference, in this case to the struct, that owns it 

    fn iz_square(&self) -> bool {  // -> specifies return type
        // we need to know the size at compile time  
        self.widht == self.height  // basic equivalence
    }
}

fn main() {
    let p: Point = Point {x: 4.2, y: 4.2};
            // p is of data type Point, rect is of data type Rect
    let rect:Rect = Rect {
        widht: 42, height: 42
    };
    println!("point {}, {}", p.x, p.y);  // here using the structure of a custom type 
           
    println!("area: {}", rect.area());   // here calling a method of a custom type 
           
    println!("is square: {}", rect.iz_square()); // here calling another method of the same type 

    // this does not have inheritance because rust does not have inheritance 
    // but we can define shared behaviour with a trait 
    polymorphism(); // look down 
           
}

struct Circle {
    radius: f32,
}

struct Square {
    side: f32,
}

// this is basically just an interface 
trait Shape {
    fn area(&self) -> f32;
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        2.0 * 3.14 * self.radius   // 2 pi r
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
}

fn polymorphism() {
    let shapes: Vec<Box<dyn Shape>> = vec![ 
        // vector of <Box<dyn Shape>, each Box holds Shape "object"
        Box::new(Circle { radius: 4.2 }),   // <Box<dyn Shape> is a fat pointer
        Box::new(Square { side: 4.2 }),
    ];

    //iters over the vec 
    for shape in shapes {
        //dynamically calls the area method because of the object type 
        println!("area: {}", shape.area());
    }
}
