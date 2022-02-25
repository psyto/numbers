pub fn print(limit: u8) {
    let numbers = generate_sequnce(limit);
    output_sequence(&numbers);
}

fn generate_sequnce(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}

fn output_sequence(numbers: &[u8]) {
    for n in numbers {
        println!("{}", n);
    }
}

#[test]
fn generate_sequnce_should_work() {
    let result = generate_sequnce(3);
    assert_eq!(result, &[1, 2, 3]);
}
