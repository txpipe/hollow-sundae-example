mod ddk;

use crate::ddk::*;

struct MyParams {
    sell_price: f32,
    sell_quantity: u64,
    max_slippage: f32,
}

struct MyDatum {
    pool_id: Vec<u8>,
}

fn on_some_utxo(utxo: Utxo<MyDatum>, config: Config<MyParams>) -> Result<(), HandleError> {
    // This is where your logic goes. Do whatever you need with the utxo and config values.

    pubsub::publish_msg("sundae.xxx", &Vec::from(b"hello"))?;

    Ok(())
}

wit_bindgen::generate!({
    world: "bod",
});

struct MyBod;

impl Guest for MyBod {
    fn handle(evt: Event) -> Result<(), HandleError> {
        Router::on_utxo()
            .at_address("addr1xxx")
            .holding_token("asset1xxxx")
            .handle_with(on_some_utxo)
            .bind(evt)
    }
}

export!(MyBod);
