pub fn print(s: &str) {
    println!("EB_NORDPOOL_DEBUG\n{}", s);
}

pub struct Debug<'a> {
    pub file: &'a str,
    pub msg: &'a str,
}

impl <'a>Debug<'a> {
    pub fn new(file: &'a str, msg: &'a str) -> Self {
        Self {
            file,
            msg,
        }
    }

    pub fn print(&self) {
        println!("EB_NORDPOOL_DEBUG");
        println!("{}", self.file);
        println!("{}", self.msg);
    }
}
