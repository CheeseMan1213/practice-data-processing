mod quadratic_formula;
mod math;
use math::add::add;
use math::multiply::multiply;
use quadratic_formula::quadratic_formula::QuadraticFormula;
fn main() {
    println!("Hello, world from backend.");
    println!("{}", add(2, 3));

    let qf = QuadraticFormula::new(1.0, -3.0, 2.0);
    let ans = qf.calculate();
    println!("{:?}", ans);
    
    println!("{}", multiply(2, 3));
}


#[cfg(test)]
mod test;
