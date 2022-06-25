use std::fmt;

// full code at https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html

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

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{} + {}i", self.real, self.imag)
    }
}

pub fn hw_display() {
    let point = Point2D { x: 3.3, y: 7.2 };
    let complex = Complex{ real: 3.3, imag: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}
