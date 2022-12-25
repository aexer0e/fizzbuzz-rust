struct FizzbuzzDataPoint {
    word: &'static str,
    condition: fn(u32) -> bool
}
const FIZZBUZZ_DATA: [FizzbuzzDataPoint; 2] = [
    FizzbuzzDataPoint {
        word: "Fizz",
        condition: |n| n % 3 == 0
    },
    FizzbuzzDataPoint {
        word: "Buzz",
        condition: |n| n % 5 == 0
    }
];

fn main() {
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
    for i in 1..n {
        fizzbuzz(i)
    }
}

fn fizzbuzz(n: u32) {
    let mut mystr: String = String::new();

    for point in FIZZBUZZ_DATA.iter() {
        if (point.condition)(n) {
            mystr.push_str(point.word);
        }
    }

    if mystr.len() == 0 {
        mystr = n.to_string()
    }

    println!("{mystr}" )
}