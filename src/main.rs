
mod crawler;
mod links_and_files;


fn main() {
	let depth :u8 = 2;
	let domain = "https://www.dawn.com";
	let _ = tokio::runtime::Runtime::new().unwrap().block_on(crawler::run_crawler(depth, domain));
    
}
