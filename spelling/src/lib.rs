pub fn spell(n: u64) -> String {
    fn under_20(n: u64) -> &'static str {
        match n {
            0 => "zero",
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
            _ => unreachable!(),
        }
    }

    fn tens_word(n: u64) -> &'static str {
        match n {
            20 => "twenty",
            30 => "thirty",
            40 => "forty",
            50 => "fifty",
            60 => "sixty",
            70 => "seventy",
            80 => "eighty",
            90 => "ninety",
            _ => unreachable!(),
        }
    }

    fn spell_under_100(n: u64) -> String {
        if n < 20 {
            under_20(n).to_string()
        } else {
            let tens = (n / 10) * 10;
            let ones = n % 10;
            if ones == 0 {
                tens_word(tens).to_string()
            } else {
                format!("{}-{}", tens_word(tens), under_20(ones))
            }
        }
    }

    fn spell_under_1000(n: u64) -> String {
        if n < 100 {
            return spell_under_100(n);
        }
        let hundreds = n / 100;
        let rem = n % 100;
        let mut s = format!("{} hundred", under_20(hundreds));
        if rem > 0 {
            s.push(' ');
            s.push_str(&spell_under_100(rem));
        }
        s
    }

    match n {
        0 => "zero".to_string(),
        1_000_000 => "one million".to_string(),
        _ if n < 1000 => spell_under_1000(n),
        _ => {
            let thousands = n / 1000;
            let rem = n % 1000;
            let mut s = format!("{} thousand", spell_under_1000(thousands));
            if rem > 0 {
                s.push(' ');
                s.push_str(&spell_under_1000(rem));
            }
            s
        }
    }
}
