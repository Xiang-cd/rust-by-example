use std::fmt;

fn display_section() {
    #[derive(Debug)]
    struct Structure2(i32);

    impl fmt::Display for Structure2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "structure value:{}", self.0);
        }
    }
    println!("This struct `{}` print with {{}}", Structure2(3));
    println!("This struct `{:?}` print with {{:?}}", Structure2(3));

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "min: {}, max: {}", self.0, self.1);
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            return write!(f, "x: {}, y: {}", self.x, self.y);
        }
    }

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Dispaly {}", point);
    println!("Debug: {:?}", point);

    #[derive(Debug)]
    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (i, v) in vec.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }
            return write!(f, "]");
        }
    }
    let v = List(vec![1, 2, 46, 24]);
    println!("Display list: {}", v);
    println!("Debug List: {:?}", v);

    #[derive(Debug)]
    struct City {
        name: &'static str,
        lat: f32, //latitude
        lon: f32, // longitude
    }
    impl fmt::Display for City {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
            return write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            );
        }
    }
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.913869,
            lon: 10.752245,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ] {
        println!("{}", city);
        println!("{:?}", city);
    }
}

fn basic_print() {
    println!("Hello, world!, this is comment section");
    let x = 5 + /* 90 +  */ 5;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("\n\n\nnow we comes to the formated print section");

    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "your", "my");

    println!(
        "{subject} {verb} {object}",
        object = "this is object",
        subject = "this is subject",
        verb = "this is verb"
    );
    const WIDTH: usize = 30;
    println!("The value of WIDTH is: {}", WIDTH);
    println!("Base 10: {:>WIDTH$}", 69420, WIDTH = WIDTH);
    println!("Base  8: {:>WIDTH$o}", 69420, WIDTH = WIDTH);
    println!("Base 16: {:>WIDTH$x}", 69420, WIDTH = WIDTH);
    println!("Base  2: {:>WIDTH$b}", 69420, WIDTH = WIDTH);

    println!("with prefix filling");
    println!("Base 10: {:>0WIDTH$}", 69420, WIDTH = WIDTH);
    println!("my name is {0}, {1}, {0}", "bond", "alice");

    println!("\n\n\nnow we comes to the debug print section");
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);
    let structure = Structure(3);
    let deep = Deep(Structure(10));
    println!("This struct `{:?}` print with {{:?}}", structure);
    println!("This struct `{:?}` print with {{:?}}", deep);
    println!("This struct `{:#?}` print with {{:?}}", deep);

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    let peter = Person {
        name: "Peter",
        age: 28,
    };
    println!("{:#?}", peter);
    println!("{:?}", peter);
}


pub fn hello_main() {
    basic_print();
    println!("\n\n\nnow we comes to the display print section");
    display_section();
}
