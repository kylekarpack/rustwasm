extern crate cfg_if;
extern crate reqwest;
extern crate select;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use select::document::Document;
use select::predicate::Name;
use wasm_bindgen::prelude::*;

cfg_if! {
		// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
		// allocator.
		if #[cfg(feature = "wee_alloc")] {
				extern crate wee_alloc;
				#[global_allocator]
				static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
		}
}

#[wasm_bindgen]
pub async fn greet() -> String {
	let url = "https://www.lexico.com/en/definition/sound";

	let mut resp = reqwest::get(url).await.text().await;
	return resp;
	// assert!(resp.status().is_success());

	// Document::from_read(resp)
	// 	.unwrap()
	// 	.find(Name("a"))
	// 	.filter_map(|n| n.attr("href"))
	// 	.for_each(|x| println!("{}", x));
	// "Hello, wasm-worker modified!".to_string()
}
