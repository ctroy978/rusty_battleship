use std::process::Command;

pub fn alpha_to_digit(alpha: &String) -> usize{

        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        letters.find(alpha).unwrap() as usize
}

pub fn clear_screen(){
    //clears the  screen
    let output = Command::new("clear").output().unwrap_or_else(|e|{
        panic!("faled to execute command: {}", e)
    });
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
