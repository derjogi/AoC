mod aux;
mod bin;

fn main() {
    // Theoretically this can be used to just call all the bin.days.
    // Practically won't be using that because with the macro in lib.rs
    // we can just run each day individually, which is what I prefer for now.
    println!("Invoke each day manually.");
}
