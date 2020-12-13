fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!")

    let x = 5 + 90 + 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{} days", 31);
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>0width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct `{}` won't print...", Structure(3));
    let pi = 3.141592;
    println!("{:.2}", pi);

    struct UnPrintable(i32);

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in year.", 12);
    println!("{1} {0:?} is the {actor:?} name.",
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
    println!("{:#?}", peter);

    use std::fmt;

    struct Structure(i32);

    impl fmt::Display for Structure {
        fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
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

    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =     MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big}, and small is {small}",
        small = small_range,
        big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

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

    let complex = Complex{real: 3.3, imag: 7.2};
    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    use std::fmt;
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {write!(f, ", ")?;}
                write!(f, "{}: {}", count, v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    use std::fmt::{self, Formatter, Display};

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
            let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

            write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
        }
    }
    #[derive(Debug)]
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    for city in [
        City { name: "Dublin", lat: 53.3477778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat:49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90},
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{:?}", *color);
    }
}
