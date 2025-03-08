pub mod complex;

use complex::C;
use complex::add_c;
use complex::mul_c;
use complex::exp_c;

use std::f64::consts::PI;

pub fn fft(x: &[f64], n: usize) -> Vec<C>
{
    let mut vec: Vec<C> = Vec::new();
    for i in 0..n
    {
        vec.push(C(x[i], 0.0));
    }
    fftr(&mut vec, n);
    vec
}

fn fftr(x: &mut Vec<C>, n: usize)
{
    if n <= 1 { return; } //base case
    
    let mut odd: Vec<C> = vec![C(0.0, 0.0); n/2];
    let mut even: Vec<C> = vec![C(0.0, 0.0); n/2];

    for i in 0..n/2
    {
        even[i] = x[i * 2]; //split into even elements (2i)
        odd[i] = x[i * 2 + 1]; //split into odd elements (2i+1)
    }

    fftr(&mut even, n/2);
    fftr(&mut odd, n/2);

    //clever symmetry trick
    for k in 0..n/2
    {
        let t: C = mul_c( exp_c(C(0.0, -2.0 * PI * (k as f64 / n as f64) )) , odd[k] );
        x[k] = add_c( even[k] , t );
        x[n/2 + k] = add_c( even[k] , C(-t.0, -t.1) );
    }
}
