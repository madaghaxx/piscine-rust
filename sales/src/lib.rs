#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for (i, j) in &s.products {
            if i.to_string() == ele {
                self.items.push((i.to_string(), *j));
                self.receipt.push(*j);
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut res = vec![];
        for i in &self.receipt {
            res.push(*i);
        }
        res
    }
}
