
use robots_txt::{Robots, matcher::SimpleMatcher, parse};
use reqwest::{self, header::HeaderValue};
use scraper::{Html, Selector};
use url::Url;
use reqwest::{Response, Request};

// static ROBOTS: &'static str = r#"

// # robots.txt for http://www.site.com
// User-Agent: *
// Disallow: /cyberworld/map/ # this is an infinite virtual URL space
// # Cybermapper knows where to go
// User-Agent: cybermapper
// Disallow:

// "#;
static ROBOT_TXT: &'static str = "https://dawn.com/robots.txt";

struct Crawler<'z> {
    robots: Robots<'z>,
    allowed_links: Vec<Url>,
    disallowed_links: Vec<Url>,
}

// async fn get_robots_txt(domain: &str) -> String {
// 	// Fetch and parse the robots.txt file for the domain
// 	let robots_url = format!("{}/robots.txt", domain);
// 	let response = get_request(&Url::parse(&robots_url).unwrap()).await.unwrap();
// 	let body = response.text().await.unwrap();
// 	body
// }

impl<'z> Crawler<'z> {
    fn new() -> Self {
        Self {
            robots: Robots::from_str_lossy(ROBOT_TXT),
            allowed_links: Vec::new(),
            disallowed_links: Vec::new(),
        }
    }

	
	


    async fn crawl(&mut self, url: &Url) {
        // Make a request to the page.
        let response = get_request(url).await.unwrap();
		

        // Parse the links on the page.
        let links = response.headers().get("Link").unwrap();
		let links = Self::parse_link_header(links);
        for link in links.iter() {
            // Parse the link into a Url object.
            let link_url = Url::parse(link).unwrap();

            // Check if the link is allowed to be crawled.
			let rules = &self.robots.choose_section("*").rules;
			let matcher = SimpleMatcher::new(&rules);
			
			
            if matcher.check_path(&link_url.as_str()) {
                // Add the link to the allowed vector.
                self.allowed_links.push(link_url);
            } else {
                // Add the link to the disallowed vector.
                self.disallowed_links.push(link_url);
            }
        }
    }

    fn get_allowed_links(&self) -> &Vec<Url> {
        &self.allowed_links
    }

    fn get_disallowed_links(&self) -> &Vec<Url> {
        &self.disallowed_links
    }

	fn parse_link_header(header: &HeaderValue) -> Vec<String> {
        let header_str = header.to_str().unwrap();//a,b,c,d,e
        let links: Vec<&str> = header_str.split(',').collect();// <google.com>| <yt.com>| <fb.com>
        let mut result = Vec::new();
        for link in links {// <google.com>, <yt.com>, <fb.com>
            let trimmed_link = link.trim();
            let url_start = trimmed_link.find('<').unwrap() + 1;
            let url_end = trimmed_link.find('>').unwrap();
            let url = &trimmed_link[url_start..url_end];
            result.push(url.to_string());
        }
        result
    }
}


async fn get_request(url: &Url) -> Option<Response> {
    let response = reqwest::get(url.as_str()).await.unwrap();

    if response.status().is_success() {
        Some(response)
    } else {
        println!("Request failed with status: {:?}", response.status());
        None
    }
}

async fn run_crawler() {
	// get_request().await;
    let mut crawler = Crawler::new();

    // Add the starting URL to the queue of URLs to crawl.
    crawler.crawl(&Url::parse("https://google.com/robots.txt").unwrap()).await;

    // Get the allowed and disallowed links.
    let allowed_links = crawler.get_allowed_links();
    let disallowed_links = crawler.get_disallowed_links();

    // Print the allowed links to the console.
	println!("allowed links: ");
    for link in allowed_links {
        println!("{}", link);
    }

    // Print the disallowed links to the console.
	println!("disallowed links: ");
    for link in disallowed_links {
        println!("{}", link);
    }
}

fn main() {
    // let mut crawler = Crawler::new();

    // Add the starting URL to the queue of URLs to crawl.
    // crawler.crawl(&Url::parse("https://example.com").unwrap()).await;
	
    // Get the allowed and disallowed links.
    // let allowed_links = crawler.get_allowed_links();
    // let disallowed_links = crawler.get_disallowed_links();

    // // Print the allowed links to the console.
    // for link in allowed_links {
	// 	println!("{}", link);
    // }
	
    // // Print the disallowed links to the console.
    // for link in disallowed_links {
	// 	println!("{}", link);
    // }
	// tokio::runtime::Runtime::new().unwrap().block_on(run_crawler());
	// get_request().await;
	// let url = Url::parse("https://dawn.com").unwrap();
	let url = Url::parse("https://www.dawn.com/news/1782012/abrahams-seed").unwrap();
	let my_vec = tokio::runtime::Runtime::new().unwrap().block_on(get_links_from_website(url)).unwrap();
	
	let robots = Robots::from_str_lossy(ROBOT_TXT);
	let rules = &robots.choose_section("*").rules;
	let matcher = SimpleMatcher::new(&rules);

	let mut allowed_links = Vec::new();
	let mut disallowed_links = Vec::new();
	
	println!("links: ");
	for link in my_vec {
		let curr_link = link.clone();
		let allowed = matcher.check_path(&link.as_str());
		if allowed {
			// Add the link to the allowed vector.
			allowed_links.push(link);
		} else {
			// Add the link to the disallowed vector.
			disallowed_links.push(link);
		}
		
		println!("{} {}", allowed, curr_link);
	
	}
}
fn find_links_in_html(html: &str) -> Vec<String> {//html: &str
	// let html = r#"
	// 	<html>
	// 	<head>
	// 	<title>Example</title>
	// 	</head>
	// 	<body>
	// 	<a href="https://google.com">Google</a>
	// 	<a href="https://example.com">Example</a>
	// 	</body>
	// 	</html>
	// "#;

	// let response = match tokio::runtime::Runtime::new().unwrap().block_on(get_request_2(&Url::parse("https://google.com").unwrap())) {
	// 	 Some(res)=> res, 
	// 	 None => panic!("Error"),
	// };

	// let html = tokio::runtime::Runtime::new().unwrap().block_on(response.text()).unwrap();

	
	let mut links = Vec::new();

    let parser = Html::parse_document(html);
    let selector = Selector::parse("a").expect("Could not parse selector");
	// let s = selector.attr("href");
    for element in parser.select(&selector) {
        links.push(element.value().attr("href").unwrap().to_string());
    }

    links

}

async fn get_request_2(url: &Url) -> Option<Response> {
    let response = reqwest::get(url.as_str()).await.unwrap();

    if response.status().is_success() {
        Some(response)
    } else {
        println!("Request failed with status: {:?}", response.status());
        None
    }
}

// use scraper::{Html, Selector};
use reqwest::{Client};
// use url::Url;

async fn get_links_from_website(url: Url) -> Option<Vec<String>> {
    // let client = Client::new();
    // let response = client.get(url);

	let client = Client::new();

    let request = Request::new(reqwest::Method::GET, url);
    let response = client.execute(request).await.unwrap();

    if response.status().is_success() {
        let html = response.text().await.unwrap();
        let links = find_links_in_html(&html);

        Some(links)
    } else {
        None
    }
}
