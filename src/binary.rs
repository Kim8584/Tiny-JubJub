pub struct Binary;
impl Binary {
    //this is use to get the binary representation of a number in the prime feild of 13
    pub fn to_binary(number: i32) -> Vec<u8> {
        let mut number = number;
        let mut bin: Vec<u8> = Vec::new();
        while number > 0 {
            if number % 2 == 0 {
                bin.push(0);
            } else {
                bin.push(1);
            }
            number /= 2;
        }
        // reverse the order of items in the list
        return bin.iter().copied().rev().collect();
    }
}
