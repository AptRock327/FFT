#[derive(Clone)]
#[derive(Copy)]
pub struct C(pub f64, pub f64);

pub fn add_c(a: C, b: C) -> C
{
    C(a.0 + b.0, a.1 + b.1)
}

pub fn mul_c(a: C, b: C) -> C
{
    C(a.0 * b.0 - a.1 * b.1, a.0 * b.1 + a.1 * b.0)
}

pub fn exp_c(x: C) -> C
{
    C(x.0.exp() * x.1.cos(), x.0.exp() * x.1.sin())
}

pub fn display_c(x: &[C], n: usize)
{
    for i in 0..n
    {
        if x[i].1 >= 0.0 { println!("{0} + {1}i", x[i].0, x[i].1); }
        else { println!("{0} - {1}i", x[i].0, -x[i].1); }
    }
}
