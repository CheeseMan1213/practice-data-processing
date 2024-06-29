mod print_count_to_ten;

fn main() {
    println!("Hello, world from frontend.");
    println!("{}", subtract(2, 3));
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod test;
