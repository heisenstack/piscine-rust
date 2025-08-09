pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut parts = Vec::new();

    let million = n / 1_000_000;
    let thousand = (n % 1_000_000) / 1_000;
    let rest = n % 1_000;

    if million > 0 {
        parts.push(format!("{} million", spell_below_thousand(million)));
    }

    if thousand > 0 {
        parts.push(format!("{} thousand", spell_below_thousand(thousand)));
    }

    if rest > 0 {
        parts.push(spell_below_thousand(rest));
    }

    parts.join(" ")
}

fn spell_below_thousand(n: u64) -> String {
    let mut parts = Vec::new();

    let hundred = n / 100;
    let rest = n % 100;

    if hundred > 0 {
        parts.push(format!("{} hundred", spell_small(hundred)));
    }

    if rest > 0 {
        parts.push(spell_tens(rest));
    }

    parts.join(" ")
}

fn spell_tens(n: u64) -> String {
    if n <= 20 {
        return spell_small(n);
    }

    let tens = n / 10 * 10;
    let units = n % 10;

    if units == 0 {
        spell_small(tens)
    } else {
        format!("{}-{}", spell_small(tens), spell_small(units))
    }
}

fn spell_small(n: u64) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        30 => "thirty",
        40 => "forty",
        50 => "fifty",
        60 => "sixty",
        70 => "seventy",
        80 => "eighty",
        90 => "ninety",
        _ => "", 
    }
    .to_string()
}
