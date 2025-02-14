mod circuits;
fn main() {
    println!("Hello, layern!");
    circuits::addProof::build_addition_circuit().unwrap();
}
