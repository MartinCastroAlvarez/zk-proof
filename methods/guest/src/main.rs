#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // Read the Fibonacci position from the host.
    let n: u64 = env::read();
    
    // Ensure the input is at least 1.
    if n < 1 {
        panic!("Input must be at least 1");
    }
    
    // Compute the Fibonacci number.
    let fib = if n == 1 || n == 2 {
        1
    } else {
        let mut a: u64 = 1; // Fibonacci(1)
        let mut b: u64 = 1; // Fibonacci(2)
        for _ in 3..=n {
            let c = a.checked_add(b).expect("Integer overflow");
            a = b;
            b = c;
        }
        b
    };

    // Commit the Fibonacci number as the guest's output.
    env::commit(&fib);
}
