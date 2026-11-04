struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 550000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850000,
        },
    ];

    let mut grand_total = 0;

    for laptop in &laptops {
        let cost = laptop.calculate_cost(3);

        println!("{}: N{}", laptop.brand, cost);

        grand_total += cost;
    }

    println!("Grand Total: N{}", grand_total);
}