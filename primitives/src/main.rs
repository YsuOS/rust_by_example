fn main() {
    let logical: bool = true;
    
    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer = 5i32;  // Suffix annotation

    let default_float = 3.0; // f64
    let default_integer = 7; // i32

    let mut inferred_type = 12; // Type i64 is inferred from "another line"
    inferred_type = 44893859838i64;

    let mut mutable = 12;
    mutable = 21;

    // mutable = true  // A type of a variable can't be changed
    
    let mutable = true;  // Variables can be overwritten with shadowing.

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 + 2);

    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    println!("One million is written as {}", 1_000_000u32);

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;

        (boolean, integer)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);


    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5u32,)); // the comma is required to tell them apart from a literal
    println!("just an integer: {:?}", (5u32));  // a literal

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    //activities
    use std::fmt;
    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
        }
    }
    println!("{}", matrix);

    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }

    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));



}
