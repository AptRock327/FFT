mod fft;

use fft::complex::display_c;
use fft::fft;

fn main()
{
    let example: [f64; 8] = [3.7, 2.1, 1.3, -5.2, -9.1, 2.4, 7.8, -1.1];
    let dft = fft(&example, 8);

    display_c(&dft, 8);
}
