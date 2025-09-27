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
        for (name, price) in &s.products {
            if *name == ele {
                self.items.push((name.clone(), *price));
                break;
            }
        }
    }
    
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let total_items = prices.len();
        let free_items = total_items / 3;
        
        if free_items > 0 {
            let total_price: f32 = prices.iter().sum();
            let free_price: f32 = prices.iter().take(free_items).sum();
            let adjusted_total = total_price - free_price;
            
            let ratio = adjusted_total / total_price;
            prices = prices.into_iter().map(|price| (price * ratio * 100.0).round() / 100.0).collect();
        }
        
        self.receipt = prices.clone();
        prices
    }
}