extern crate cfg_if;
extern crate reqwest;
extern crate select;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use reqwest::Error;
use select::document::Document;
use select::predicate::Name;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{self, spawn_local};

cfg_if! {
		// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
		// allocator.
		if #[cfg(feature = "wee_alloc")] {
				extern crate wee_alloc;
				#[global_allocator]
				static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
		}
}

async fn greet() -> Result<(), > {
  let res = reqwest::get("https://www.rust-lang.org/en-US/")
    .await?
    .text()
    .await?;

  Document::from(res.as_str())
    .find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| println!("{}", x));

  Ok(())
}

async fn get() -> Result<(), Error> {
	let request_url = format!(
		"https://api.github.com/repos/{owner}/{repo}/stargazers",
		owner = "rust-lang-nursery",
		repo = "rust-cookbook"
	);
	println!("{}", request_url);
	let response = reqwest::get(&request_url).await?;

	let users = response.text().await?;
	println!("{:?}", users);
	Ok(())
}
