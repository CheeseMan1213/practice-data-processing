mod quadratic_formula;
mod add;
use add::add::add;
use quadratic_formula::quadratic_formula::QuadraticFormula;
fn main() {
    println!("Hello, world from backend.");
    println!("{}", add(2, 3));

    let qf = QuadraticFormula::new(1.0, -3.0, 2.0);
    let ans = qf.calculate();
    println!("{:?}", ans);
}


#[cfg(test)]
mod test;
