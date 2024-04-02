use std::collections::HashMap;
use std::fmt;
pub struct Header {
    path: String,
    host: String,
    map: HashMap<String, String>,
}

impl Header {
    pub fn new(path: &str, host: &str) -> Header {
        let map = HashMap::new();
        Header {
            path: path.to_owned(),
            host: host.to_owned(),
            map,
        }
    }
    pub fn add(mut self, key: &str, value: &str) -> Header {
        self.map.insert(key.to_owned(), value.to_owned());
        self
    }
    pub fn remove(mut self, key: &str) -> Header {
        self.map.remove(key);
        self
    }
    pub fn to_string(&self) -> String {
        format!("{self}")
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rt = format!("GET {} HTTP/1.0\r\nHost: {}\r\n", self.path, self.host);
        for (key, value) in self.map.iter() {
            rt = rt + &format!("{}: {}\r\n", key, value);
        }
        rt = rt + "\r\n";
        write!(f, "{}", rt)
    }
}
