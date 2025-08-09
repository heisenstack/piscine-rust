pub fn talking(text: &str) -> &str {
   const REPLY_1: &str = "There is no need to yell, calm down!";
   const REPLY_2: &str = "Sure.";
   const REPLY_3: &str = "Quiet, I am thinking!";
   const REPLY_4: &str = "Just say something!";
   const REPLY_5: &str = "Interesting";
   println!("This is the text: {text}");

 match text {
     _ if text.trim().is_empty() => REPLY_4,
    _ if text.ends_with("?") && is_all_uppercased(&text) => REPLY_3,
    _ if is_all_uppercased(&text) => REPLY_1,
    _ if text.ends_with("?") => REPLY_2,
    _ => REPLY_5,
}

}

pub fn is_all_uppercased(text: &str) -> bool {
    if text.is_empty() || text.chars().all(|c| !c.is_alphabetic()) {
        return false;
    }

    text.to_ascii_uppercase() == text && text.chars().any(|c| c.is_alphabetic())
}