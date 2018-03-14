extern crate tau;
extern crate num_complex;

use num_complex::Complex;

fn main() {
    println!("τ = {}", tau::TAU);

    let radius: f64 = 15.0;
    println!("Circle circumference = τ * r = {}", tau::TAU * radius);
    let c: Complex<f64> = Complex::from_polar(&1.0, &tau::TAU);
    println!("Euler's identity: exp(i * τ) = {}", c);
    println!("Trigonometry: sin(τ) = {}, cos(τ) = {}", tau::TAU.sin(), tau::TAU.cos());

    println!("That other constant = {}", tau::TAU / 2.0);
}
