use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, SubAssign, MulAssign};

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}

macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

trait JustNumbers{}
//other types can also be added
impl JustNumbers for i32 {}
impl JustNumbers for f64 {}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex{
    real:f64,
    imag:f64,
}

impl Complex{
    fn new<R, I>(real:R, imag:I)->Complex
    where
        f64:From<R> + From<I>
    {
        Complex { real: f64::from(real), imag: f64::from(imag) }
    }

    fn conjugate(&self) -> Complex{
        Complex { real: f64::from(self.real), imag: f64::from(-self.imag) }
    }
}

impl From<i32> for Complex
{
    fn from(value:i32)->Self{
        Complex { real: value as f64, imag: 0.0 }
    }
}

impl From<f64> for Complex
{
    fn from(value:f64)->Self{
        Complex { real: value, imag: 0.0 }
    }
}

impl Add<Complex> for Complex
{
    type Output = Complex;

    fn add(self, rhs: Complex)-> Self::Output
    where
        Complex:Copy
    {
        Complex{real:self.real + rhs.real, imag:self.imag + rhs.imag}
    }
}

impl<T> Add<T> for Complex
where 
    T:JustNumbers,
    f64:From<T>
{
    type Output = Complex;

    fn add(self, rhs: T)-> Self::Output
    {
        Complex{real:self.real + <f64>::from(rhs), imag:self.imag}
    }
}

impl AddAssign<Complex> for Complex
{
    fn add_assign(&mut self, rhs: Complex)
    {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl<T> AddAssign<T> for Complex
where
    T:JustNumbers,
    f64:From<T>
{
    fn add_assign(&mut self, rhs: T) {
        self.real += <f64>::from(rhs);
    }
}

impl Sub<Complex> for Complex
{
    type Output = Complex;

    fn sub(self, rhs: Complex)-> Self::Output
    where
        Complex:Copy
    {
        Complex{real:self.real - rhs.real, imag:self.imag - rhs.imag}
    }
}

impl<T> Sub<T> for Complex
where 
    T:JustNumbers,
    f64:From<T>
{
    type Output = Complex;

    fn sub(self, rhs: T)-> Self::Output
    {
        Complex{real:self.real - <f64>::from(rhs), imag:self.imag}
    }
}

impl SubAssign<Complex> for Complex
{
    fn sub_assign(&mut self, rhs: Complex)
    {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl<T> SubAssign<T> for Complex
where
    T:JustNumbers,
    f64:From<T>
{
    fn sub_assign(&mut self, rhs: T) {
        self.real -= <f64>::from(rhs);
    }
}

impl Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex)-> Self::Output
    where
        Complex:Copy
    {
        Complex{real:self.real*rhs.real - self.imag*rhs.imag, imag:self.imag*rhs.real + self.real*rhs.imag}
    }
}

impl<T> Mul<T> for Complex
where 
    T:JustNumbers + Copy,
    f64:From<T>
{
    type Output = Complex;

    fn mul(self, rhs: T)-> Self::Output
    where
    {
        Complex{real:self.real*<f64>::from(rhs) , imag:self.imag*<f64>::from(rhs)}
    }
}

impl MulAssign<Complex> for Complex
{
    fn mul_assign(&mut self, rhs: Complex)
    {
        let real = self.real;
        self.real = self.real*rhs.real - self.imag*rhs.imag;
        self.imag = self.imag*rhs.real + real*rhs.imag;
    }
}

impl<T> MulAssign<T> for Complex
where
    T:JustNumbers + Copy,
    f64:From<T>
{
    fn mul_assign(&mut self, rhs: T) {
        self.real = self.real*<f64>::from(rhs);
        self.imag = self.imag*<f64>::from(rhs);
    }
}

impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Display for Complex
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.imag > 0.0 && self.real != 0.0 {
            write!(f, "{}+{}i", self.real, self.imag)
        } 
        else if self.imag < 0.0 && self.real != 0.0 {
            write!(f, "{}{}i", self.real, self.imag)
        }
        else if self.imag > 0.0 && self.real == 0.0 {
            write!(f, "{}i", self.imag)
        }
        else if self.imag < 0.0 && self.real == 0.0 {
            write!(f, "-{}i", self.imag)
        }
        else if self.imag == 0.0 && self.real > 0.0 {
            write!(f, "{}", self.real)
        }
        else if self.imag == 0.0 && self.real < 0.0{
            write!(f, "-{}", self.real)
        }
        else {
            write!(f, "0")
        }
    }
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);
    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");

    //Bonus
    let mut a1 = Complex::new(4.0, 7);
    a1 += Complex::new(1, 3);
    assert_eq_rel!(a1.real, 5);
    assert_eq_rel!(a1.imag, 10);
    a1 += a1;
    assert_eq_rel!(a1.real, 10);
    assert_eq_rel!(a1.imag, 20);
    a1 += 1;
    a1 += 1.0;
    assert_eq_rel!(a1.real, 12);
    assert_eq_rel!(a1.imag, 20);

    a1 = Complex::new(4.0, 7);
    a1 -= Complex::new(1, 3);
    assert_eq_rel!(a1.real, 3);
    assert_eq_rel!(a1.imag, 4);
    a1 -= a1;
    assert_eq_rel!(a1.real, 0);
    assert_eq_rel!(a1.imag, 0);
    a1 -= 1;
    a1 -= 1.0;
    assert_eq_rel!(a1.real, -2);
    assert_eq_rel!(a1.imag, 0);

    a1 = Complex::new(4.0, 7);
    a1 *= Complex::new(1, 3);
    assert_eq_rel!(a1.real, -17);
    assert_eq_rel!(a1.imag, 19);
    a1 *= 1;
    a1 *= 1.0;
    assert_eq_rel!(a1.real, -17);
    assert_eq_rel!(a1.imag, 19);

    println!("ok bonus!");
    
}
