const NUM_MATCHES:i32 = 14; //constant

fn main(){
    let season:&str = "Libertadores 2012"; //string variable
    let mut goals:i32 = 22; //integer mutable variable
    println!("In {}, Corinthians scored {} goals throught the {} matches!", season, goals, NUM_MATCHES);
    goals = 5; //update the mutable variable
    println!("Emerson Sheik, Corinthians' top scorer, had {} goals!", goals);
}