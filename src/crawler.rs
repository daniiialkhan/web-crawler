use csv::Writer;
use url::Url;

use crate::{links_and_files::links_and_files::{get_timestamp, get_robots_txt, create_file}, crawler::crawler::Crawler};



pub mod crawler{
	//use std::{time::{SystemTime, UNIX_EPOCH}, fs::File};
	
	use robots_txt::{Robots, matcher::SimpleMatcher, parse};
	use reqwest::{self, header::HeaderValue, Client};
	use scraper::{Html, Selector};
	use url::Url;
	use reqwest::{Response, Request};
	use csv::Writer;
	use std::thread;
	use std::sync::Mutex;

	use crate::links_and_files::links_and_files::*;
	pub struct Crawler<'z> {
		pub robots: Robots<'z>,
		pub allowed_links: Vec<String>,
		pub disallowed_links: Vec<String>,
		pub domain: &'z str,                // to store domain name of the website (stays till lifetime exists= meaning till the struck exists)
	}
	impl<'z> Crawler<'z> {
		pub fn new(domain: &'z str, robots_txt: &'z str) -> Self {
			Self {
				robots: Robots::from_str_lossy(robots_txt),
				allowed_links: Vec::new(),
				disallowed_links: Vec::new(),
				domain,														// save the domain name
			}
		}

		pub fn allowed_links(&self) -> &Vec<String> {
			&self.allowed_links
		}

		pub fn disallowed_links(&self) -> &Vec<String> {
			&self.disallowed_links
		}
		
	
		pub async fn crawl(&mut self, url: Url, timestamp: u64) {
			// Get the links from the page.
			let links = self.make_request(url).await.unwrap_or_else(||{
				println!("Error making request");
				Vec::new()
			});//.unwrap_or_else(Vec::new);
	
			// println!("links: ");
	
			let rules = &self.robots.choose_section("*").rules;
			let matcher = SimpleMatcher::new(&rules);
	
			let raw_links = create_file("raw","raw_links".to_string(), timestamp).unwrap();
	
			let mut writer = Writer::from_writer(raw_links);
	
			writer.write_record(&["No.", "allowed", "link"]).expect("Error writing record");
	
			let mut i = 1;
	
			for link in links {
				let curr_link = link.clone();
				let allowed = matcher.check_path(&link.as_str());
	
				if allowed {
					// Add the link to the allowed vector.
					self.allowed_links.push(link);
				} else {
					// Add the link to the disallowed vector.
					self.disallowed_links.push(link);
				}
				
				// println!("{} {}", allowed, curr_link);
	
				writer.write_record(&[i.to_string(), allowed.to_string(), curr_link]).expect("Error writing record");
				i += 1;
	
			
			}
			writer.flush().expect("Error flushing writer");

		}

		
	pub async fn make_request(&mut self, url: Url) -> Option<Vec<String>> {
		
		// Make a request to the URL, receieve a response.
		let client = Client::new();
		let request = Request::new(reqwest::Method::GET, url);
		let response = match client.execute(request).await {
			Ok(response) => {
				println!("Response received successfully===");
				response
			},
			Err(_) => {
				println!("Error making request to url====");
				return None
			},
		};

		// If the response is successful, get the links from the page.
		if response.status().is_success() {
			let html = response.text().await.unwrap_or_else(|_| {
				println!("Error getting text from response");
				"".to_string()
			});
			let links = self.links_from_html(&html);

			Some(links)
		} else {
			None
		}

	}

		fn links_from_html(&mut self, html: &str) -> Vec<String> {
			
			let mut links = Vec::new();
			
			let parser = Html::parse_document(html);
			let selector = Selector::parse("a").expect("Could not parse selector");
			
			for element in parser.select(&selector) {
				links.push(element.value().attr("href").unwrap().to_string());
			}
		
			links
			
		}
		
	}
	
}

pub async fn run_crawler(depth :u8) -> Result<(), std::io::Error> {

	if depth == 0 {
		return Ok(());
	}
	
	let timestamp = get_timestamp();

	let domain = "https://www.dawn.com";
	let news = "https://www.dawn.com/news";

	let robots_txt = get_robots_txt(domain).await; // get robots txt for the domain

	let mut crawler = Crawler::new(domain,&robots_txt);


	let filtered_links = create_file("depth_1","filtered_links".to_string(), timestamp).unwrap();

	let mut writer = Writer::from_writer(filtered_links);

	writer.write_record(&["No.", "link"]).expect("Error writing record");

	// Set the starting URL
	crawler.crawl(Url::parse("https://www.dawn.com/").unwrap(), timestamp).await;
	// crawler.crawl(Url::parse("https://www.dawn.com/news/1781789/hamas-releases-2-us-hostages-for-humanitarian-reasons").unwrap(), timestamp).await;

	let url_queue = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();

	println!("\n\nurl_queue: {:#?}\n\n", url_queue);

	let mut i = 1;
	
	for url in url_queue.iter() {
		writer.write_record(&[i.to_string(), url.to_string()]).expect("Error writing record");
		i += 1;
	}
	
	writer.flush().expect("Error flushing writer");
	
	if depth == 1 {
		return Ok(());
	}
	i = 1;
	
	// make threads for each url in url_queue

	

	for url in url_queue.iter() {
		println!("\n\ncrawling url: {}", url);
		let mut crawler = Crawler::new(domain,&robots_txt);
		crawler.crawl(Url::parse(url).unwrap(), timestamp).await;

		let url_list = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();

		println!("\n\nurl_list: {:#?}\n\n", url_list);

		let mut j = 1;
		
		let filtered_links = create_file("depth_2",format!("filtered_links_{i}"), timestamp).unwrap();

		let mut writer = Writer::from_writer(filtered_links);
		
	
		writer.write_record(&["No.", "link"]).expect("Error writing record");

		for link in url_list.iter() {
			writer.write_record(&[j.to_string(), link.to_string()]).expect("Error writing record");
			j += 1;
		}

		writer.flush().expect("Error flushing writer");
		i += 1;

		if depth == 3 {
			j = 1;
	
			for url in url_list.iter() {
				println!("\n\ncrawling url: {}", url);
				let mut crawler = Crawler::new(domain,&robots_txt);
				crawler.crawl(Url::parse(url).unwrap(), timestamp).await;
		
				let url_list = crawler.allowed_links().clone().iter().filter(|&x| {println!("filter x: {x}");x.contains(news)}).map(|x|{println!("map x: {x}"); x.to_string()}).collect::<Vec<String>>();
		
				println!("\n\nurl_list: {:#?}\n\n", url_list);
		
				let mut k = 1;
				
				let filtered_links = create_file("depth_3",format!("filtered_links_{j}"), timestamp).unwrap();
		
				let mut writer = Writer::from_writer(filtered_links);
				
			
				writer.write_record(&["No.", "link"]).expect("Error writing record");
		
				for link in url_list.iter() {
					writer.write_record(&[j.to_string(), link.to_string()]).expect("Error writing record");
					k += 1;
				}
		
				writer.flush().expect("Error flushing writer");
				j += 1;
		
			}

		}

	}
	
	Ok(())
}