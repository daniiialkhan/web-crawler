// use std::{time::{SystemTime, UNIX_EPOCH}, fs::File};

// use robots_txt::{Robots, matcher::SimpleMatcher, parse};
// use reqwest::{self, header::HeaderValue, Client};
// use scraper::{Html, Selector};
// use url::Url;
// use reqwest::{Response, Request};
// use csv::Writer;
// use std::thread;
// use std::sync::Mutex;
// modules
// mod crawler;
// use crawler::Crawler;
// use crate::crawler::*;
// use crawler::run_crawler;
mod crawler;


mod links_and_files;
// use links_and_files;


fn main() {
	let depth :u8 = 2;
	let _ = tokio::runtime::Runtime::new().unwrap().block_on(crawler::run_crawler(depth));
    
}

// static ROBOTS: &'static str = r#"

// # robots.txt for http://www.site.com
// User-Agent: *
// Disallow: /cyberworld/map/ # this is an infinite virtual URL space
// # Cybermapper knows where to go
// User-Agent: cybermapper
// Disallow:

// "#;
//static ROBOT_TXT: &'static str = "https://www.dawn.com/robots.txt";

// struct Crawler<'z> {
//     robots: Robots<'z>,
//     allowed_links: Vec<String>,
//     disallowed_links: Vec<String>,
// 	domain: &'z str,                // to store domain name of the website (stays till lifetime exists= meaning till the struck exists)
// }

// // Returns the robots.txt file for the domain. 
// async fn get_robots_txt(domain: &str) -> String {
// 	// Fetch and parse the robots.txt file for the domain
// 	let robots_url = format!("{}/robots.txt", domain);
// 	let response = get_response(&Url::parse(&robots_url).unwrap()).await.unwrap();
// 	// let response = get_response(&Url::parse(&robots_url).unwrap()).await.unwrap();
// 	let body = response.text().await.unwrap();
// 	// let body = response.text().await.unwrap().as_str();
// 	body
// }

// async fn get_response(url: &Url) -> Option<Response> {
// 	let client = Client::new();

// 	let request = Request::new(reqwest::Method::GET, url.clone());
// 	let response = client.execute(request).await.unwrap();

// 	if response.status().is_success() {
// 		Some(response)
// 	} else {
// 		None
// 	}
// }

// impl<'z> Crawler<'z> {
//     fn new(domain: &'z str, robots_txt: &'z str) -> Self {
		

//         Self {
//             robots: Robots::from_str_lossy(robots_txt),
//             allowed_links: Vec::new(),
//             disallowed_links: Vec::new(),
// 			domain,														// save the domain name
//         }
//     }

//     async fn crawl(&mut self, url: Url, timestamp: u64) {
//         // Get the links from the page.
// 		let links = self.make_request(url).await.unwrap_or_else(||{
// 			println!("Error making request");
// 			Vec::new()
// 		});//.unwrap_or_else(Vec::new);

// 		// println!("links: ");

// 		let rules = &self.robots.choose_section("*").rules;
// 		let matcher = SimpleMatcher::new(&rules);

// 		let raw_links = create_file("raw","raw_links".to_string(), timestamp).unwrap();

// 		let mut writer = Writer::from_writer(raw_links);

// 		writer.write_record(&["No.", "allowed", "link"]).expect("Error writing record");

// 		let mut i = 1;

// 		for link in links {
// 			let curr_link = link.clone();
// 			let allowed = matcher.check_path(&link.as_str());

// 			if allowed {
// 				// Add the link to the allowed vector.
// 				self.allowed_links.push(link);
// 			} else {
// 				// Add the link to the disallowed vector.
// 				self.disallowed_links.push(link);
// 			}
			
// 			// println!("{} {}", allowed, curr_link);

// 			writer.write_record(&[i.to_string(), allowed.to_string(), curr_link]).expect("Error writing record");
// 			i += 1;

		
// 		}
// 		writer.flush().expect("Error flushing writer");


// 		// for link in links.iter() {
//         //     // Parse the link into a Url object.
//         //     let link_url = Url::parse(link).unwrap();

//         //     // Check if the link is allowed to be crawled.
// 		// 	let rules = &self.robots.choose_section("*").rules;
// 		// 	let matcher = SimpleMatcher::new(&rules);
			
			
//         //     if matcher.check_path(&link_url.as_str()) {
//         //         // Add the link to the allowed vector.
//         //         self.allowed_links.push(link_url);
//         //     } else {
//         //         // Add the link to the disallowed vector.
//         //         self.disallowed_links.push(link_url);
//         //     }
//         // }
//     }

	
	
	
    // fn allowed_links(&self) -> &Vec<String> {
	// 	&self.allowed_links
    // }

    // fn disallowed_links(&self) -> &Vec<String> {
    //     &self.disallowed_links
    // }
	
	// fn links_from_html(&mut self, html: &str) -> Vec<String> {
		
	// 	let mut links = Vec::new();
		
	// 	let parser = Html::parse_document(html);
	// 	let selector = Selector::parse("a").expect("Could not parse selector");
		
	// 	for element in parser.select(&selector) {
	// 		links.push(element.value().attr("href").unwrap().to_string());
	// 	}
	
	// 	links
		
	// }
	// async fn make_request(&mut self, url: Url) -> Option<Vec<String>> {
		
	// 	// Make a request to the URL, receieve a response.
	// 	let client = Client::new();
	// 	let request = Request::new(reqwest::Method::GET, url);
	// 	let response = match client.execute(request).await {
	// 		Ok(response) => {
	// 			println!("Response received successfully===");
	// 			response
	// 		},
	// 		Err(_) => {
	// 			println!("Error making request to url====");
	// 			return None
	// 		},
	// 	};

	// 	// If the response is successful, get the links from the page.
	// 	if response.status().is_success() {
	// 		let html = response.text().await.unwrap_or_else(|_| {
	// 			println!("Error getting text from response");
	// 			"".to_string()
	// 		});
	// 		let links = self.links_from_html(&html);

	// 		Some(links)
	// 	} else {
	// 		None
	// 	}

	// }
	// fn parse_link_header(header: &HeaderValue) -> Vec<String> {
    //     let header_str = header.to_str().unwrap();//a,b,c,d,e
    //     let links: Vec<&str> = header_str.split(',').collect();// <google.com>| <yt.com>| <fb.com>
    //     let mut result = Vec::new();
    //     for link in links {// <google.com>, <yt.com>, <fb.com>
    //         let trimmed_link = link.trim();
    //         let url_start = trimmed_link.find('<').unwrap() + 1;
    //         let url_end = trimmed_link.find('>').unwrap();
    //         let url = &trimmed_link[url_start..url_end];
    //         result.push(url.to_string());
    //     }
    //     result
    // }
// }
// fn get_timestamp() -> u64 {
// 	let now = SystemTime::now();
// 	let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
// 	since_epoch.as_secs()
// }

// fn create_file(path: &str, filename: String, timestamp: u64) -> Result<File, std::io::Error> {
// 	// let timestamp = get_timestamp();

// 	match std::fs::create_dir_all(format!("./urls/{}/{}", timestamp, path)) {
// 		Ok(()) => {
// 			println!("Directory created successfully");
// 			let mut filepath = "".to_string();
// 			if (path == "".to_string()) {
// 				filepath = format!("./urls/{}/{}.csv", timestamp, filename);
// 			}
// 			else {
// 				filepath = format!("./urls/{}/{}/{}.csv", timestamp, path,  filename);
// 			}
// 			match File::create(filepath) {
// 				Ok(file) => {
// 					println!("File created successfully");
// 					Ok(file)
// 				},
// 				Err(e) => {
// 					println!("Error creating file: {}", e);
// 					Err(e)
// 				},
// 			}

// 		},
// 		Err(e) => {
// 			println!("Error creating directory: {}", e);
// 			Err(e)
// 		},
// 	}

// }

// async fn run_crawler(depth :u8) -> Result<(), std::io::Error> {

// 	if depth == 0 {
// 		return Ok(());
// 	}
	
// 	let timestamp = get_timestamp();

// 	let domain = "https://www.dawn.com";
// 	let news = "https://www.dawn.com/news";

// 	let robots_txt = get_robots_txt(domain).await; // get robots txt for the domain

//     let mut crawler = Crawler::new(domain,&robots_txt);


// 	let filtered_links = create_file("depth_1","filtered_links".to_string(), timestamp).unwrap();

// 	let mut writer = Writer::from_writer(filtered_links);

// 	writer.write_record(&["No.", "link"]).expect("Error writing record");

// 	// Set the starting URL
// 	crawler.crawl(Url::parse("https://www.dawn.com/").unwrap(), timestamp).await;
// 	// crawler.crawl(Url::parse("https://www.dawn.com/news/1781789/hamas-releases-2-us-hostages-for-humanitarian-reasons").unwrap(), timestamp).await;

// 	let url_queue = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();

// 	println!("\n\nurl_queue: {:#?}\n\n", url_queue);

// 	let mut i = 1;
	
// 	for url in url_queue.iter() {
// 		writer.write_record(&[i.to_string(), url.to_string()]).expect("Error writing record");
// 		i += 1;
// 	}
	
// 	writer.flush().expect("Error flushing writer");
	
// 	if depth == 1 {
// 		return Ok(());
// 	}
// 	i = 1;
	
// 	// make threads for each url in url_queue

	

// 	for url in url_queue.iter() {
// 		println!("\n\ncrawling url: {}", url);
// 		let mut crawler = Crawler::new(domain,&robots_txt);
// 		crawler.crawl(Url::parse(url).unwrap(), timestamp).await;

// 		let url_list = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();

// 		println!("\n\nurl_list: {:#?}\n\n", url_list);

// 		let mut j = 1;
		
// 		let filtered_links = create_file("depth_2",format!("filtered_links_{i}"), timestamp).unwrap();

// 		let mut writer = Writer::from_writer(filtered_links);
		
	
// 		writer.write_record(&["No.", "link"]).expect("Error writing record");

// 		for link in url_list.iter() {
// 			writer.write_record(&[j.to_string(), link.to_string()]).expect("Error writing record");
// 			j += 1;
// 		}

// 		writer.flush().expect("Error flushing writer");
// 		i += 1;

// 		if depth == 3 {
// 			j = 1;
	
// 			for url in url_list.iter() {
// 				println!("\n\ncrawling url: {}", url);
// 				let mut crawler = Crawler::new(domain,&robots_txt);
// 				crawler.crawl(Url::parse(url).unwrap(), timestamp).await;
		
// 				let url_list = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();
		
// 				println!("\n\nurl_list: {:#?}\n\n", url_list);
		
// 				let mut k = 1;
				
// 				let filtered_links = create_file("depth_3",format!("filtered_links_{j}"), timestamp).unwrap();
		
// 				let mut writer = Writer::from_writer(filtered_links);
				
			
// 				writer.write_record(&["No.", "link"]).expect("Error writing record");
		
// 				for link in url_list.iter() {
// 					writer.write_record(&[j.to_string(), link.to_string()]).expect("Error writing record");
// 					k += 1;
// 				}
		
// 				writer.flush().expect("Error flushing writer");
// 				j += 1;
		
// 			}

// 		}

// 	}
	
// 	Ok(())
// }


// depth = 1 :
//  go thru only the url that i give      √

// depth = 2 :
//  go thru the url that i give and the links on that page

// depth = 3 :
//  go thru the url that i give, the links on that page and the links on the links on that page


// 1. user / main enters the domain name 													√
// 2. set the robots.txt																	√
// 3. reqwest part to get a response from url												√
// 4. parse the response to get the links													√
// 5. check if the links are allowed or not													√
// 6. if allowed, add to allowed links vector, otherwise add to disallowed links vector		(useless but √)
// 7. print the allowed and disallowed links vector											(√)
// 8. repeat from step 3 for all the links in the allowed links vector                     -- depth
///////////////////////////////////////////////////////////////////////////////////////////////////