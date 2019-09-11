fn main() {
    //let upper_limit = 120;
    //let primes = sieve_of_eratosthenes_prime_generator(upper_limit);
    //println!("Primes below {}:\n", upper_limit);
    //for i in primes.iter() {
    //    print!("{} ", i);
    //}
    let number_to_test = 67280421310721;
    println!("Is {} prime? {}", number_to_test, 
    if trial_division_optimized_primality_test(number_to_test) { "Yes" } else { "No" });
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

fn trial_division_optimized_primality_test(value: usize) -> bool {
    if value <= 3 {
        return value > 1
    } else if value % 2 == 0 || value % 3 == 0 {
        return false;
    } else {
        let mut divisor = 5;
        while divisor * divisor <= value {
            if value % divisor == 0 || value % (divisor + 2) == 0 {
                return false;
            }
            divisor += 6;
        }

        return true;
    }
}