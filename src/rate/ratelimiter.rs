use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use info_utils::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct RateLimiter {
    pub rate: u32,
    map: HashMap<String, DateTime<Utc>>
}

impl RateLimiter {
    pub fn add(&mut self, ip: String) {
        let now = Utc::now();
        self.map.entry(ip).or_insert(now);

        log!("{:#?}", self);
    }
}
