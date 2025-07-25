pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials: Vec<String> = Vec::new();
    
    for name in names {
        let mut temp = String::new();
        let mut found_space = false;
        
        for (i, c) in name.chars().enumerate() {
            if i == 0 {
                temp.push(c);
                temp.push_str(". ");
                
            }
            if c == ' ' {
                found_space = true
            }else if found_space {
                temp.push(c);
                temp.push_str(".");
                found_space = false;
            }
        } 
        initials.push(temp.clone());
    }
    // println!("{:?}", initials)
    initials
}


fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
} 
