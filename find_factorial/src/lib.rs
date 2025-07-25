pub fn factorial(num: u64) -> u64 {
    if num == 0 {
        return 1;
    }
    let mut temp: u64 = 1;
    let mut res: u64 = num;
    while temp < num {
        res *= temp;
        temp += 1;
    }
    res
}
