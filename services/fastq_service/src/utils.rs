use crate::schema::MinIOStructuredUrl;
use regex::Regex;

pub fn parse_url(url: &str) -> Option<MinIOStructuredUrl> {
    // This might not be the perfect regex, but probably works for now.
    let re = Regex::new(r"^https?://[^/]+/(?<bucket>[a-zA-Z0-9\-]+)/(?<key>.+)$").ok()?;

    let captures = re.captures(url)?;

    Some(MinIOStructuredUrl {
        bucket: captures.name("bucket")?.as_str().to_string(),
        key: captures.name("key")?.as_str().to_string(),
    })
}
