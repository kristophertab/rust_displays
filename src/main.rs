use std::fmt;

#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    println!("Stuent's complex number:");

    let x = Complex {
        real: 2.0,
        imag: 3.5,
    };

    println!("Display {}", x);
    println!("Debug {:?}", x);
}
