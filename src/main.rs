// // use reqwest;
// // use tokio;

// // async fn get() -> Result<(), reqwest::Error> {
// // 	let response = reqwest::get("https://example.com")
// // 		.await?;

// // 	if response.status().is_success() {
// // 		let body = response.text().await?;
// // 		println!("{}", body);
// // 	} else {
// // 		println!("Request failed with status: {:?}", response.status());
// // 	}

// // 	Ok(())
// // }

// // // #[tokio::main]
// // fn main() -> Result<(), reqwest::Error> {
// // 	tokio::runtime::Builder::new_current_thread()
// // 		.enable_all()
// // 		.build()
// // 		.unwrap()
// // 		.block_on(get())	
// // }
// use reqwest::blocking::Client;
// use url::Url;
// // use robots::RobotsTxt;
// use robots_txt::Robots;

// struct Crawler {
//     client: Client,
//     robots_txt: Robots<'static>,
// }

// impl Crawler {
//     fn new() -> Self {
//         Self {
//             client: Client::new(),
//             robots_txt: Robots::new("https://example.com/robots.txt"),
//         }
//     }

//     fn crawl(&mut self, url: &Url) {
//         // Check the robots.txt file to see if we are allowed to crawl this page.
//         if !self.robots_txt.is_allowed(url) {
//             return;
//         }

//         // Make a request to the page.
//         let response = self.client.get(url).unwrap();

//         // Parse the links on the page.
//         let links = response.headers().get("Link").unwrap();
//         for link in links.iter() {
//             // Parse the link into a Url object.
//             let link_url = Url::parse(link).unwrap();

//             // Add the link to the queue of URLs to crawl.
//             self.crawl(&link_url);
//         }
//     }
// }

// fn main() {
//     let mut crawler = Crawler::new();

//     // Add the starting URL to the queue of URLs to crawl.
//     crawler.crawl(&Url::parse("https://example.com").unwrap());

//     // Start crawling the web.
//     // crawler.crawl();
// }

// use robots_txt::{Robots, matcher::SimpleMatcher};

// static ROBOTS: &'static str = r#"

// # robots.txt for http://www.site.com
// User-Agent: *
// Disallow: /cyberworld/map/ # this is an infinite virtual URL space
// # Cybermapper knows where to go
// User-Agent: cybermapper
// Disallow:

// "#;

// fn main() {
//     let robots = Robots::from_str_lossy(ROBOTS);

//     let matcher = SimpleMatcher::new(&robots.choose_section("NoName Bot").rules);
//     println!("{}",matcher.check_path("/some/page"));
//     println!("{}",matcher.check_path("/cyberworld/welcome.html"));
//     println!("{}",matcher.check_path("/cyberworld/map/object.html"));

//     let matcher = SimpleMatcher::new(&robots.choose_section("Mozilla/5.0; CyberMapper v. 3.14").rules);
//     println!("{}",matcher.check_path("/some/page"));
//     println!("{}",matcher.check_path("/cyberworld/welcome.html"));
//     println!("{}",matcher.check_path("/cyberworld/map/object.html"));
// }


use robots_txt::{Robots, matcher::SimpleMatcher};
use reqwest::{self, header::HeaderValue};
use url::Url;

static ROBOTS: &'static str = r#"

# robots.txt for http://www.site.com
User-Agent: *
Disallow: /cyberworld/map/ # this is an infinite virtual URL space
# Cybermapper knows where to go
User-Agent: cybermapper
Disallow:

"#;

struct Crawler<'z> {
    robots: Robots<'z>,
    allowed_links: Vec<Url>,
    disallowed_links: Vec<Url>,
}

impl<'z> Crawler<'z> {
    fn new() -> Self {
        Self {
            robots: Robots::from_str_lossy(ROBOTS),
            allowed_links: Vec::new(),
            disallowed_links: Vec::new(),
        }
    }

    async fn crawl(&mut self, url: &Url) {
        // Make a request to the page.
        let response = reqwest::get(url.as_str()).await.unwrap();

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

fn main() {
    let mut crawler = Crawler::new();

    // Add the starting URL to the queue of URLs to crawl.
    crawler.crawl(&Url::parse("https://example.com").unwrap());

    // Get the allowed and disallowed links.
    let allowed_links = crawler.get_allowed_links();
    let disallowed_links = crawler.get_disallowed_links();

    // Print the allowed links to the console.
    for link in allowed_links {
        println!("{}", link);
    }

    // Print the disallowed links to the console.
    for link in disallowed_links {
        println!("{}", link);
    }
}
