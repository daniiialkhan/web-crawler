//pub static ROBOT_TXT: &'static str = "https://www.dawn.com/robots.txt";
pub mod links_and_files{
    //use reqwest::{Client, Request};
    use url::Url;
	use std::{time::{SystemTime, UNIX_EPOCH}, fs::File};

	
	use reqwest::{self, Client};
	use reqwest::{Response, Request};


	// Returns the robots.txt file for the domain. 
	pub async fn get_robots_txt(domain: &str) -> String {
		// Fetch and parse the robots.txt file for the domain
		let robots_url = format!("{}/robots.txt", domain);
		let response = get_response(&Url::parse(&robots_url).unwrap()).await.unwrap();
		// let response = get_response(&Url::parse(&robots_url).unwrap()).await.unwrap();
		let body = response.text().await.unwrap();
		// let body = response.text().await.unwrap().as_str();
		body
	}

	pub async fn get_response(url: &Url) -> Option<Response> {
		let client = Client::new();

		let request = Request::new(reqwest::Method::GET, url.clone());
		let response = client.execute(request).await.unwrap();

		if response.status().is_success() {
			Some(response)
		} else {
			None
		}
	}

	pub fn get_timestamp() -> u64 {
		let now = SystemTime::now();
		let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
		since_epoch.as_secs()
	}
	
	pub fn create_file(path: &str, filename: String, timestamp: u64) -> Result<File, std::io::Error> {
		// let timestamp = get_timestamp();
	
		match std::fs::create_dir_all(format!("./urls/{}/{}", timestamp, path)) {
			Ok(()) => {
				println!("Directory created successfully");
				let mut filepath = "".to_string();
				if path == "".to_string() {
					filepath = format!("./urls/{}/{}.csv", timestamp, filename);
				}
				else {
					filepath = format!("./urls/{}/{}/{}.csv", timestamp, path,  filename);
				}
				match File::create(filepath) {
					Ok(file) => {
						println!("File created successfully");
						Ok(file)
					},
					Err(e) => {
						println!("Error creating file: {}", e);
						Err(e)
					},
				}
	
			},
			Err(e) => {
				println!("Error creating directory: {}", e);
				Err(e)
			},
		}
	
	}

	
}