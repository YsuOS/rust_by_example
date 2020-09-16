#![allow(dead_code)]

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck! I'm putting this snake back in the forest."),
        Some(inner) => println!("{}? How nice.", inner),
        None    => println!("No gift? Oh well"),
    }
}

fn give_princess(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAaaaaa!!!!"); }
    println!("I love {}s!!!!", inside);
}

struct Person {
    job: Option<Job>,
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}


#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

fn peel(food: Option<Food>)  -> Option<Peeled> {
   match food {
       Some(food) => Some(Peeled(food)),
       None => None,
   } 
}

fn chop(peeled: Option<Peeled>)  -> Option<Chopped> {
   match peeled {
       Some(Peeled(food)) => Some(Chopped(food)),
       None => None,
   } 
}

fn cook(chopped: Option<Chopped>)  -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>)  -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh no! It wasn't edible."),
    }
}

#[derive(Debug)] enum Food2 { CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day { Monday, Tuesday, Wednesday }

fn have_ingredients(food: Food2) -> Option<Food2> {
    match food {
        Food2::Sushi => None,
        _   => Some(food),
    } 
}

fn have_recipe(food: Food2) -> Option<Food2> {
    match food {
        Food2::CordonBleu => None,
        _   => Some(food),
    }
}

fn cookable_v1(food: Food2) -> Option<Food2> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        }
    }
}

fn cookable_v2(food: Food2) -> Option<Food2> {
    have_recipe(food).and_then(have_ingredients)
}

//fn cookable_v2(food: Food2) -> Option<Option<Food2>> {   // nested Option
//    have_recipe(food).map(have_ingredients)
//}

fn eat2(food: Food2, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None    => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

use std::num::ParseIntError;

fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => {
            match second_number_str.parse::<i32>() {
                Ok(second_number) => {
                    Ok(first_number * second_number)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

type AliasedResult<T> = Result<T, ParseIntError>;
fn multiply3(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
    })
}

fn print2(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn multiply4(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number) => first_number,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number) => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn multiply5(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}

fn multiply6(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = try!(first_number_str.parse::<i32>());
    let second_number = try!(second_number_str.parse::<i32>());

    Ok(first_number * second_number)
}

fn double_first(vec: Vec<&str>)  -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn double_first2(vec: Vec<&str>)  -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

fn double_first3(vec: Vec<&str>)  -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });
    opt.map_or(Ok(None), |r| r.map(Some))
}

use std::fmt;

type Result2<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct DoubleError;

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first4(vec: Vec<&str>) -> Result2<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn print3(result: Result2<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

use std::error;

type Result3<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for EmptyVec {}

fn double_first5(vec: Vec<&str>) -> Result3<i32> {
    vec.first()
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|e| e.into())
                .map(|i| 2 * i)
        })
}

fn print4(result: Result3<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn double_first6(vec: Vec<&str>) -> Result3<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

type Result4<T> = std::result::Result<T, DoubleError2>;

#[derive(Debug)]
enum DoubleError2 {
    EmptyVec2,
    Parse(ParseIntError),
}

impl fmt::Display for DoubleError2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DoubleError2::EmptyVec2 => 
                write!(f, "please use a vector with at least one element"),
            DoubleError2::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for DoubleError2 {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DoubleError2::EmptyVec2 => None,
            DoubleError2::Parse(ref e) => Some(e),
        }
    }
    
}

impl From<ParseIntError> for DoubleError2 {
    fn from(err: ParseIntError) -> DoubleError2 {
        DoubleError2::Parse(err) 
    }
}

fn double_first7(vec: Vec<&str>) -> Result4<i32> {
    let first = vec.first().ok_or(DoubleError2::EmptyVec2)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print5(result: Result4<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}


fn main() -> Result<(), ParseIntError> {
    let food  = Some("cabbage");
    let snake = Some("snake");
    let void  = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
//    let nothing = None;

    give_princess(bird);
//    give_princess(nothing);

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));


    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);


    let (cordon_bleu, steak, sushi) = (Food2::CordonBleu, Food2::Steak, Food2::Sushi);

    eat2(cordon_bleu, Day::Monday);
    eat2(steak, Day::Tuesday);
    eat2(sushi, Day::Wednesday);

    let twenty = multiply("10", "2");
    println!("double is {}", twenty);
//    let tt = multiply("t", "2");
//    println!("double is {}", tt);

    let twenty = multiply2("10", "2");
    print(twenty);
//    let tt = multiply2("t", "2");
//    print(tt);

    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number) => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);

    print2(multiply3("10", "2"));
//    print2(multiply3("t", "2"));

    print(multiply4("10", "2"));
    print(multiply4("t", "2"));

    print(multiply5("10", "2"));
    print(multiply5("t", "2"));

    print(multiply6("10", "2"));
    print(multiply6("t", "2"));

 
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

//    println!("The first doubled is {}", double_first(empty));
//    // Error 1: the input vector is empty
//
//    println!("The first doubled is {}", double_first(strings));
//    // Error 2: the element doesn't parse to a number

    let numbers = vec!["42", "93", "18"];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first2(numbers));
    println!("The first doubled is {:?}", double_first2(empty));
    println!("The first doubled is {:?}", double_first2(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {:?}", double_first3(numbers));
    println!("The first doubled is {:?}", double_first3(empty));
    println!("The first doubled is {:?}", double_first3(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print3(double_first4(numbers));
    print3(double_first4(empty));
    print3(double_first4(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print4(double_first5(numbers));
    print4(double_first5(empty));
    print4(double_first5(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print4(double_first6(numbers));
    print4(double_first6(empty));
    print4(double_first6(strings));

    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print5(double_first7(numbers));
    print5(double_first7(empty));
    print5(double_first7(strings));

    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .filter_map(Result::ok)
        .collect();
    println!("Results: {:?}", numbers);

    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<_>, _> = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Results: {:?}", numbers);

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Results: {:?}", numbers);
    println!("Results: {:?}", errors);

    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Results: {:?}", numbers);
    println!("Results: {:?}", errors);

    Ok(())
}
