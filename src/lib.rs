
pub fn alpha_to_digit(alpha: &String) -> usize{

        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        letters.find(alpha).unwrap() as usize
}
