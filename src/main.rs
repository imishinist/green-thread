mod green;
const STACK_SIZE: usize = 2 * 1024 * 1024;

fn mash() {
    green::spawn(ortega, STACK_SIZE, "ortega from mash");
    for _ in 0..10 {
        println!("Mash!");
        green::schedule();
    }
}

fn ortega() {
    for _ in 0..10 {
        println!("Ortega!");
        green::schedule();
    }
}

fn gaia() {
    green::spawn(mash, STACK_SIZE, "mash from gaia");
    for _ in 0..10 {
        println!("Gaia!");
        green::schedule();
    }
}

fn main() {
    green::spawn_from_main(gaia, STACK_SIZE, "gaia from main");
}
