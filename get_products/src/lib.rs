pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.is_empty() || arr.len() == 1 {
        return vec![];
    }
    let mut res = vec![];
    for i in &arr {
        let mut temp = 1;
        for j in &arr {
            if *i != *j {
                temp *= *j;
            }
        }
        res.push(temp);
    }
    res
}
