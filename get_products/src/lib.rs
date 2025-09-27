pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res = vec![];
    for i in &arr {
        let mut temp = 1;
        for j in &arr {
            if *i != *j {
                temp *= j;
            }
        }
        res.push(temp);
    }
    res
}
