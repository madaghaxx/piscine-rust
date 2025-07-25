pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let abso: f64 = c as f64;
    (c, abso.exp(), abso.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut answer: Vec<f64> = vec![];
    for i in a.chars() {
        if i != ' ' {
            let num = i.to_string().parse::<f64>();
            if let Ok(n) = num {
                answer.push(n.exp());
            }
        }
    }
    (
        a,
        answer
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" "),
    )
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    (
        b.clone(),
        b
            .iter()
            .map(|x| (*x as f64).abs().ln())
            .collect(),
    )
}
