### Name : Rajeswari NB
### Student Id : 916809371
### CS 510 -Rust programming Winter 2020 HW1

### Step 1   install Rust
I installed rust using the command curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
It installed Cargo 1.40.0 by default

### Step 2 function Implementation
I wrote a pure function that takes 3 u64 arguments and returns a u64 integer and named the function as modexp and implemented the pseudocode inside the function

### Step 3 Testing
To test it, I wrote a test function and did assertequal test scenarios and verified the correctnes of the modexp function

### Step 4 Get input from command line
By following the steps used in the GCD program I implemented the main function to accept parameters from command prompt,parse those inputs as integer type and catch exception by calling the error function 

### Step 5 Using cargo tools
I used cargo fmt via terminal and noticed the formatting such as my inline if function 
```Rust
    if x == 0 { return 0 }
```
got formatted like 
```Rust
    if x == 0 {
        return 0;
    }
```

I then used cargo clippy and noticed no warning was thrown out onto my terminal

### Step 6 Run test
```
cargo build
cargo test
cargo run 2 20 17
```

got output as 16




