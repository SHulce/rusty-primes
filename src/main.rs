fn main() {
    //let upper_limit = 120;
    //let primes = sieve_of_eratosthenes_prime_generator(upper_limit);
    //println!("Primes below {}:\n", upper_limit);
    //for i in primes.iter() {
    //    print!("{} ", i);
    //}
    let number_to_test = 67280421310721;
    println!("Is {} prime? {}", number_to_test, 
    if trial_division_primality_test(number_to_test) { "Yes" } else { "No" });
}

fn sieve_of_eratosthenes_prime_generator(max_value: usize) -> Vec<usize> {
    let mut integer_set = Vec::new();
    let mut composite_flag = Vec::new();

    for i in 2..(max_value + 1) {
        integer_set.push(i);
        composite_flag.push(false);
    }

    for i in 0..integer_set.len() >> 1 {
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

fn trial_division_primality_test(value: usize) -> bool {
    for i in 2..((value as f64).sqrt() as usize) {
        if value % i == 0 {
            return false;
        }
    }

    return true;
}