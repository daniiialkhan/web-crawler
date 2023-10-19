use reqwest;
use tokio;

async fn get() -> Result<(), reqwest::Error> {
	let response = reqwest::get("https://example.com")
		.await?;

	if response.status().is_success() {
		let body = response.text().await?;
		println!("{}", body);
	} else {
		println!("Request failed with status: {:?}", response.status());
	}

	Ok(())
}

// #[tokio::main]
fn main() -> Result<(), reqwest::Error> {
	tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(get())	
}
