use rust_calculator::tui;

fn main() {
    loop {
        let input = tui::prompt();
        println!("You entered: {}", input);
    }
}
