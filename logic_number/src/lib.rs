pub fn number_logic(num: u32) -> bool {
    let nbr_of_digits =  nbr_of_digits(num);
    let  mut temp = num;
    let mut result =0;

    while temp > 0 {
        result += (temp%10).pow(nbr_of_digits as u32);
        temp /= 10;
    }
    // println!("{result}");
    result == num
}

pub fn nbr_of_digits(num: u32) -> i32 {
    let mut count = 0;
    let mut temp = num;
    while temp > 0 {
        temp/=10;
        count += 1;
    }
    count
}