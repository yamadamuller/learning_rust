fn is_even(number:i32) -> bool{
    let remainder:i32= number%2; //compute the remainder
    if remainder!=0 {
        return false //an odd number divided by two has remainder
    }
    else {
        return true //an even number divided by two has no remainder
    }
}

fn main() {
    let arg_1:i32 = 8;
    let rem_1:bool = is_even(arg_1);
    println!("{arg_1} -> {rem_1}");
    let arg_2:i32 = 9;
    let rem_2:bool = is_even(arg_2);
    println!("{arg_2} -> {rem_2}");
}
