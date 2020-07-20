fn main() {
    println!("Hello, world!");
    
    // Activiy 1
    println!("I'm a Rustacean!");

    // Line comments which go to the end of the line
    /* Block comments which go to the closing delimiter. */

    println!("{}", format!("test {}", "format"));
    print!("test {}\n", "print");

    eprint!("test {}\n", "eprint");
    eprintln!("test {}", "eprintln");

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // :b formats to binary
    // {:?} marker is for debug purposes
    // {} marker formats text in a more elegant, user friendly fashion
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "_____1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // pad numbers with extra zeroes.
    println!("{number:>0width$}", number=1, width=6);

    // Create a structure named `Structure` which contains an `i32`
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // Activities 2
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    //println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    //print 3.142
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);
    println!("Pi is roughly {:.1$}", pi, 4);

    //All types which want to use std::fmt formatting traits require an 
    //implementation to be printable. Automatic implementations are only 
    //provided for types such as in the std library. 
    //All others must be manually implemented somehow.

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor="actor's");

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};

    // {:#?} is pretty printing
    println!("{:#?}", peter);

    use std::fmt;

    #[derive(Debug)]
    struct MinMax(i64, i64);

    // To use the `{}` marker, the trait `fmt::Display` must be implemented
    // manually for the type.
    impl fmt::Display for MinMax {
        // This trait requires `fmt` with "this exact signature".
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    let minmax = MinMax(0,14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare structures:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    //activity
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex { real: 3.3, imag: 7.2};

    println!("Compare structures:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    //activity

    struct List(Vec<i32>);
    
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(f, "{}: {:.3}°{} {:.3}°{}", 
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }

    //activity
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let hex = format!("{:02X}{:02X}{:02X}", self.red, self.green, self.blue);

            write!(f, "RGB ({}, {}, {}) 0x{}", 
                self.red, self.green, self.blue, hex)
        }
    }

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
//        println!("{:?}", *color);
    }
}
