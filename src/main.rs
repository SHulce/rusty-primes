fn main() {
    let upper_limit = 300000000;
    let primes = prime_generator(upper_limit);
    println!("Primes below {}:\n", upper_limit);
    for i in primes.iter() {
        print!("{} ", i);
    }
}

fn prime_generator(max_value: usize) -> Vec<usize> {
    let mut integer_set = Vec::new();
    let mut composite_flag = Vec::new();

    for i in 2..(max_value + 1) {
        integer_set.push(i);
        composite_flag.push(false);
    }

    for i in 0..integer_set.len() {
        let mut current_multiple = 2;
        let current_value = integer_set[i];
        loop {
            let value_being_checked = current_value * current_multiple;
            if value_being_checked >= (max_value + 1) {
                break;
            }
            composite_flag[value_being_checked - 2] = true;
            current_multiple += 1;
        }
    }

    let mut discovered_primes: Vec<usize> = Vec::new();

    for i in 0..integer_set.len() {
        if !(composite_flag[i]) {
            discovered_primes.push(integer_set[i]);
        }
    }

    discovered_primes
}

