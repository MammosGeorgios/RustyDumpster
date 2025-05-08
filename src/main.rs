fn main() {
    static ITERATIONS: i32 = 15;

    println!("Welcome to Rust Dumper");
    println!("First method call! my_number = {}", get_a_number());
    call_a_loop(ITERATIONS);

    println!(concat!(
        "You can concat string with the macro concat! ",
        stringify!(What on earth is stringify though)
    ));
}

fn get_a_number() -> i32 {
    let my_number: i32 = 1821;
    my_number // woah look at this return
}

fn call_a_loop(iterations: i32) {
    for _i in 0..iterations {
        println!("Oh wow a loop, with number {}", _i);
    }
}
