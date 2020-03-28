use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

#[derive(Debug)]
pub struct Point {
    pub measurement: String,
    pub tagset: HashMap<String, String>,
    pub fieldset: HashMap<String, String>,
    pub timestamp: String,
}

impl Point {
    pub fn set_fieldset(volume: String, close: String) -> HashMap<String, String> {
        let mut foo = HashMap::new();
        foo.insert("volume".to_string(), volume);
        foo.insert("close".to_string(), close);
        foo.clone()
    }

    pub fn set_tagset() -> HashMap<String, String> {
        let mut foo = HashMap::new();
        foo.insert("frequency".to_string(), "daily".to_string());
        foo.insert("type".to_string(), "close".to_string());
        foo.clone()
    }

    pub fn get_lineprotocol(self) -> Result<String, Box<dyn std::error::Error>> {
        let mut s = String::new();
        write!(&mut s, "{},", self.measurement).expect("error in measurement");

        for (key, val) in self.tagset {
            write!(&mut s, "{}={},", key, val).expect("error in tagset");
        }

        // remove the last comma from the tagset
        let mut strlen = s.len();
        let mut s1 = String::from(s);
        s1.remove(strlen - 1);

        // add in a space between the tagset and the fieldset
        write!(&mut s1, "{}", " ".to_string()).expect("error in space");

        for (key, val) in self.fieldset {
            write!(&mut s1, "{}={},", key, val).expect("error in fieldset");
        }

        // remove the last comma from the fieldset
        strlen = s1.len();
        let mut s2 = String::from(s1);
        s2.remove(strlen - 1);

        write!(&mut s2, " {}", self.timestamp).expect("error in timestamp");
        Ok(s2)
    }
}
