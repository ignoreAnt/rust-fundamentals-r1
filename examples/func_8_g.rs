fn get_sale_price(original_price: f64, discount_percent: f64) -> f64 {
    original_price - (original_price * (discount_percent / 100.0))
}

fn main() {
    let original_price = 150.0;
    let discount_percent = 20.0;

    let final_price = get_sale_price(original_price, discount_percent);
    println!("Final price after {}% discount is: {:.2}", discount_percent, final_price);
}
