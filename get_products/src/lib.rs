pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    if arr.len() == 1 {
        return arr;
    }
    let mut res = Vec::new();
    for i in &arr {
        let mut temp = 1;
        for j in &arr {
            if j == i {
                continue;
            }
            temp *= j;
        }
        res.push(temp);
    }
    res
}
