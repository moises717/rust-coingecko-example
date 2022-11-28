use serde::{Serialize, Deserialize};

fn main() {
    let mut coin: String = String::new();
    println!("¿Qué criptomoneda quieres consultar?");
    let _ = std::io::stdin().read_line(&mut coin).expect("Ocurrió un error");

         let result_precio = get_precio(&coin);

         match result_precio {
            Ok(precio) => println!("El precio de {coin} es: {precio}"),
            Err(error) => println!("Error al buscar precio: {}", error),
        }
           
    }

fn get_precio(_coin: &str) -> Result<String, ureq::Error>{
    let body: String = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{_coin}?localization=false"))
        .set("Example-Header", "header value")
        .call()?
        .into_string()?;

        let coin_data: CoinData = serde_json::from_str(&body).unwrap();

        Ok(coin_data.market_data.current_price.usd.to_string())
    
}
#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketData

}
#[derive(Serialize, Deserialize, Debug)]
struct MarketData{
    current_price: Prices
}
#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    usd: f32
}