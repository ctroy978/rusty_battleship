
pub fn alpha_to_digit(alpha: &String) -> usize{

        let letters = "ABCDEFGHIJDLMNOPQRSTUVWXYZ";
        letters.find(alpha).unwrap() as usize
}
