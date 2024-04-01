fn main() {

    // create a guessing game:
    // 1. generate a random number
    // 2. ask the user to input a number
    // 3. compare the number with the random number
    // 4. if the number is too small, print "too small"
    // 5. if the number is too big, print "too big"
    // 6. if the number is equal, print "you win"
    // 7. if the number is not a number, print "not a number"
    // 8. if the number is not in the range [1, 100], print "out of range"
    // 9. repeat 2-8 until the user wins
    // 10. print the number of tries
    // 11. print the average number of tries

    // generate a random number
    let random_number = 42;

    // ask the user to input a number
    let mut input = String::new();
    println!("Please input a number between 1 and 100:");
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();

    // compare the number with the random number
    if input < 1 || input > 100 {
        println!("out of range");
    } else if input < random_number {
        println!("too small");
    } else if input > random_number {
        println!("too big");
    } else {
        println!("you win");
    }

    // print the number of tries
    println!("number of tries: 1");

    // print the average number of tries
    println!("average number of tries: 1");



    // optimize the spatial autocorrelation function for python:
    // this will be expand into a package for python
    // 1. create a function that takes a numpy array as input
    
    // 2. concurrently calculate the spatial autocorrelation function for each pixel

    // 3. optimize the hardware usage for multi-thread performance  (e.g. use all the cores of the CPU)

    // 4. Calculate the ram usage
    

}
