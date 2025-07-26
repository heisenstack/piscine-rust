pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut jump = false;
    let mut count = 0;
    for value in s.chars() {
        if value == '-' {
            res.pop();
            continue;
        } else if value == '+' {
            count += 1;
            jump = true;
            continue;
        }
        if jump {
            if count > 0 {
                count -= 1;
                continue;
            }
        }
        res.push(value);
        jump = false;
    }
    *s = res
}

pub fn do_operations(v: &mut [String]) {
    for n in v.iter_mut() {
        if n.contains("+") {
            let nums: Vec<_> = n.split('+').collect();
            *n = (nums[0].parse::<i32>().unwrap() + nums[1].parse::<i32>().unwrap()).to_string();
        } else {
            let nums: Vec<_> = n.split('-').collect();
            *n = (nums[0].parse::<i32>().unwrap() - nums[1].parse::<i32>().unwrap()).to_string();
        }
    }
}

fn main() {
    let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
    let mut b = ["2+2".to_owned(), "3+2".to_owned(), "10-3".to_owned(), "5+5".to_owned()];

    delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", (a, b));
}
