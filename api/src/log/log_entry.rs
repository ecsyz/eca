use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LogEntry<'s> {
    timestamp:  &'s str,
    pub action: &'s str,
    from:       &'s str,
    to:         &'s str,
    module:     &'s str,
    amount:     i64,
    hit_type:   &'s str,

    from_corp: &'s str,
    from_alli: &'s str,
    from_ship: &'s str,

    to_corp: &'s str,
    to_alli: &'s str,
    to_ship: &'s str,
}

impl<'s> LogEntry<'s> {
    pub fn new(
        timestamp: &'s str,
        action: &'s str,
        from: &'s str,
        to: &'s str,
        module: &'s str,
        amount: i64,
        hit_type: &'s str,
        from_corp: &'s str,
        from_alli: &'s str,
        from_ship: &'s str,
        to_corp: &'s str,
        to_alli: &'s str,
        to_ship: &'s str,
    ) -> Self {
        Self {
            timestamp,
            action,
            from,
            to,
            module,
            amount,
            hit_type,
            from_corp,
            from_alli,
            from_ship,
            to_corp,
            to_alli,
            to_ship,
        }
    }

    pub fn n(timestamp: &'s str) -> Self {
        Self {
            timestamp,
            ..Default::default()
        }
    }

    pub fn act(&mut self, v: &'s str) -> &mut Self {
        self.action = v;
        self
    }
    pub fn from(&mut self, v: &'s str) -> &mut Self {
        self.from = v;
        self
    }
    pub fn from_adv(&mut self, v: &'s str) -> &mut Self {
        let re = regex::Regex::new(r"^(.*)\[(.*?)\]\((.*?)\)$").unwrap();
        if let Some(cap) = re.captures(&v) {
            self.from = cap.get(1).unwrap().as_str();
            self.from_corp = cap.get(2).unwrap().as_str();
            self.from_ship = cap.get(3).unwrap().as_str();
        } else {
            // println!("RegExp: {:?}", &re);
            // println!("String: {:?}", &v);
            // panic!("from_adv parsing ERROR");
            self.from = v;
        }
        self
    }

    pub fn to(&mut self, v: &'s str) -> &mut Self {
        self.to = v;
        self
    }
    pub fn to_adv(&mut self, v: &'s str) -> &mut Self {
        let re = regex::Regex::new(r"^(.*)\[(.*?)\]\((.*?)\)$").unwrap();
        if let Some(cap) = re.captures(&v) {
            self.to = cap.get(1).unwrap().as_str();
            self.to_corp = cap.get(2).unwrap().as_str();
            self.to_ship = cap.get(3).unwrap().as_str();
        } else {
            println!("RegExp: {:?}", &re);
            println!("String: {:?}", &v);
            panic!("to_adv parsing ERROR");
        }
        self
    }
    pub fn module(&mut self, v: &'s str) -> &mut Self {
        self.module = v;
        self
    }
    pub fn amount(&mut self, v: i64) -> &mut Self {
        self.amount = v;
        self
    }
    pub fn amount_str(&mut self, v: &'s str) -> &mut Self {
        self.amount = v.parse::<i64>().unwrap();
        self
    }

    pub fn hit_type(&mut self, v: &'s str) -> &mut Self {
        self.hit_type = v;
        self
    }
}
