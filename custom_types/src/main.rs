fn main() {
    #[derive(Debug)]
    struct Person<'a> {
        // The 'a defines a lifetime
        name: &'a str,
        age: u8,
    }

    struct Unit;
    struct Pair(i32, f32);

    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields
    // of our other one
    let bottom_right = Point { x: 5.2, ..point };  // bottom_right.y = point.y
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;

    let rectangle = Rectangle {
        top_left: Point { x: top_edge, y: left_edge },
        bottom_right: bottom_right,
    };

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // activities
    fn rect_area (rect: Rectangle) -> f32 {
        let width = rect.bottom_right.x - rect.top_left.x;
        let height = rect.top_left.y - rect.bottom_right.y;
        width*height
    }
    let rect = Rectangle { 
        top_left: Point { x: 5.2, y: 10.4 },
        bottom_right: Point { x: 14.5, y: 5.3 }
    };
    println!("top_left {:?}\nbottom_right {:?}",
        rect.top_left,
        rect.bottom_right
    );
    println!("Rectangle area is {}", rect_area(rect));

    fn square (point: Point, len: f32) -> Rectangle {
        // point is a bottom_left corner of Rectangle
        Rectangle { 
            top_left: Point { x: point.x, y: point.y + len },
            bottom_right: Point { x: point.x + len, y: point.y }
        }
    }

    println!("bottom_left {:?}", point);
    let square = square(point, 3.0);
    println!("top_left {:?}\nbottom_right {:?}",
        square.top_left,
        square.bottom_right
    );
}
