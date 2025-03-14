fn recursive_factorial(num:i32)->i32{
    if num == 0{
        return 1
    }
    else{
        let out = num * recursive_factorial(num-1);
        return out
    }

}

fn recursive_fibonacci(seq:i32)->i32{
    if seq == 0{
        return 0
    }
    else if seq == 1{
        return 1
    }
    else{
        let out = recursive_fibonacci(seq-1) + recursive_fibonacci(seq-2);
        return out
    }
}

fn main() {
    //compute the factorial of an integer
    let num:i32 = 5;
    let fact_num:i32 = recursive_factorial(num);
    println!("factorial({num}) = {fact_num}");

    //compute the fibonacci sequence of an integer index
    let fibo_seq:i32 = 6;
    let fibo_num:i32 = recursive_fibonacci(fibo_seq);
    println!("fibonacci({fibo_seq}) = {fibo_num}");
}
