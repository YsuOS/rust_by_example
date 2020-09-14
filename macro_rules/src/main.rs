macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! create_function {
    // the arguments of a macro are prefixed by a dollar sign `$`
    // :ident is a designator. ident is used fo variable/function names
    ($func_name:ident) => {
        fn $func_name() {
            //stringify! macro converts an `ident` into a string
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // expr is used for expressions
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!("{:?} and {:?} is {:?}", 
            stringify!($left),
            stringify!($right),
            $left && $right);
    };
    ($left:expr; or $right:expr) => {
        println!("{:?} or {:?} is {:?}", 
            stringify!($left),
            stringify!($right),
            $left || $right);
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);

    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_min!($($y),+))
    )
}

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // tt is used for operators and tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        //`assert!`: 1st argument is a condition, 2nd and laters are for assertion
        //messages.
        assert!($a.len() == $b.len(),
           "{:?}: dimension mismatch: {:?} {:?} {:?}",
           stringify!($func),
           ($a.len(),),
           stringify!($op),
           ($b.len()),
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<T, Output=T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);

            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
            }
        }
        
    };
}

op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    use std::iter;
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);
                    assert_eq!(x, z);
                }
            }
        };
    }

    test!(add_assign, 1u32, 2u32, 3u32);
    test!(mul_assign, 2u32, 3u32, 6u32);
    test!(sub_assign, 3u32, 2u32, 1u32);

}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            {
                let val: usize = $e;
                println!("{} = {}", stringify!{$e}, val);
            }
        }
    };
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }}
}
fn main() {
    say_hello!();
    foo();
    bar();

    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);

    println!("{}", find_min!(1u32));
    println!("{}", find_min!(1u32 + 2, 2u32));
    println!("{}", find_min!(5u32, 2u32 * 3, 4u32));

    calculate! {
        eval 1 + 2
    }
    calculate! {
        eval (1 + 2) * (3 / 4)
    }

    calculate! {
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}
