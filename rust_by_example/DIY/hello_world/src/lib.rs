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


// full code at https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display/testcase_list.html



// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{c}: {}", v, c = count)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

pub fn hw_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
