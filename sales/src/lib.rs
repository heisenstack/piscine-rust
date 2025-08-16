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
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((_, price)) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();

        prices.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let mut discount_total = 0.0;
        for i in 0..prices.len() / 3 {
            discount_total += prices[i * 3 + 2];
        }

        let total: f32 = prices.iter().sum();
        let discount_percentage = if total != 0.0 { discount_total / total } else { 0.0 };

        let receipt: Vec<f32> = prices
            .iter()
            .map(|p| {
                let discounted = p * (1.0 - discount_percentage);
                (discounted * 100.0).round() / 100.0 
            })
            .collect();

        self.receipt = receipt.clone();
        self.receipt.clone()
    }
}

