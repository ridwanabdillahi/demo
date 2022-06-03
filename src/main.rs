#![feature(debugger_visualizer)]
#![debugger_visualizer(natvis_file = "demo.natvis")]
use regex::*;

fn main() {
    let builder = RegexBuilder::new(r"(?P<letters>[a-z]+)(?:([0-9]+)|([A-Z]+))");
    let re = builder.build().unwrap();

    let book = Book::new("Demo Book".to_string(), 10);
    let words = book.read_page();

    let caps = re.captures(&words[0]).unwrap();

    let matches = caps.iter().filter_map(|s| s).collect::<Vec<Match>>();
    assert_eq!(matches.len(), 3);

    let text1 = caps.get(1).map_or("", |m| m.as_str());
    let text2 = caps.get(2).map_or("", |m| m.as_str());
    let text3 = caps.get(3).map_or("", |m| m.as_str());
    assert_eq!(text1, "hello");
    assert_eq!(text2, "123");
    assert_eq!(text3, "");
    println!("Hello, world!");
}

pub struct Page {
    words: Vec<String>,
}

pub struct Book {
    title: String,
    pages: Vec<Page>,
    current_page: usize,
}

impl Book {
    pub fn new(title: String, num_pages: usize) -> Book {
        let mut pages = Vec::with_capacity(num_pages);
        for i in 0..num_pages {
            let page = Page {
                words: vec!["hello123".to_string(); i * i + 5]
            };
            pages.push(page);
        }
        Book { title, pages, current_page: 0 }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn next_page(&mut self) -> &Page {
        if self.current_page == self.pages.len() {
            panic!("No more pages to read, sorry.")
        }
        
        let next_page = &self.pages[self.current_page];
        self.current_page += 1;
        next_page
    }

    pub fn read_page(&self) -> &Vec<String> {
        &self.pages[self.current_page].words
    }

    pub fn previous_page(&mut self) -> &Page {
        if self.current_page == 0 {
            panic!("Already at the beginning of the book, sorry.")
        }
        
        let prev_page = &self.pages[self.current_page];
        self.current_page -= 1;
        prev_page
    }
}
