use regex::Regex;
use reqwest::Url;
use std::{collections::HashMap, sync::LazyLock};

use super::psendpoint::PSEndpoint;
use super::pserror::PSError;
use super::pserror::MAX_PS_FETCH_SIZE;
use super::sortopts::{Parameter, Sort};
use super::timeconvenience::TimeConvenience;

// PushShift API
static PUSHSHIFT: &str = "https://api.pushshift.io/reddit";
// I tested the RegEx below so unwrap() is fine.
static VALID_REDDIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\w\d_-]+").unwrap());

/// Builds a reqwest::Url for the PushShift Reddit API.
#[derive(Clone, Debug)]
pub struct PushshiftBuilder {
    url: String,
    params: HashMap<String, String>,
}

impl PushshiftBuilder {
    pub fn new(endpoint: PSEndpoint) -> Self {
        PushshiftBuilder {
            url: format!("{PUSHSHIFT}{endpoint}"),
            params: HashMap::new(),
        }
    }

    /// Builds the PushShift API call provided that the caller specified some parameters.
    /// This function doesn't consume self to facilitate building new URLs using _replace_sub_.
    pub fn build(&mut self) -> Result<Url, PSError> {
        // The params HashMap shouldn't be empty
        if !self.params.is_empty() {
            // Sorting is always added so we can actually paginate the results as mentioned in the
            // sort() function.
            self.sort(Sort::Desc, Parameter::CreatedUTC)
                // Finally, check for "before" and add the parameter as the max u32 value if it
                // doesn't exist. Using u32 max is safe (I checked), but the before() function
                // takes in a u64 in case the API changes. I'm not entirely sure how epochs work
                // but u64 seemed like the right idea. I check for "before" in case the caller
                // provided it already.
                .params
                .entry("before".to_owned())
                .or_insert_with(|| u32::MAX.to_string());
            Ok(Url::parse_with_params(&self.url, &self.params)?)
        } else {
            Err(PSError::NoParams)
        }
    }

    pub fn build_multiple<T: AsRef<str>>(&mut self, subs: &[T]) -> Result<Vec<Url>, PSError> {
        subs.iter()
            .map(|sub| self.replace_sub(sub.as_ref())?.build())
            .collect()
    }

    pub fn before(&mut self, time: TimeConvenience) -> Result<&mut Self, PSError> {
        self.add_param("before", &time.to_string())
    }

    pub fn after(&mut self, time: TimeConvenience) -> Result<&mut Self, PSError> {
        self.add_param("after", &time.to_string())
    }

    /// Replaces the currently defined subreddit.
    /// This function exists due to my poor API design.
    pub fn replace_sub(&mut self, sub: &str) -> Result<&mut Self, PSError> {
        let _ignore = self.params.remove("subreddit");
        self.subreddit(sub)
    }

    pub fn score_threshold(&mut self, thresh: u32) -> Result<&mut Self, PSError> {
        // Admittedly, I should handle >, <, and = but I'm too lazy right now.
        self.add_param("score", &(String::from(">") + &thresh.to_string()))
    }

    // Sort is private because PushshiftBuilder always sets sorting now.
    // The duplicate parameter error is unimportant and thus consumed.
    fn sort(&mut self, sort: Sort, by: Parameter) -> &mut Self {
        if !self.params.contains_key("sort") {
            self.add_param("sort", &sort.to_string())
                .and_then(|ps| ps.add_param("sort_type", &by.to_string()))
                .ok();
        }
        self
    }

    pub fn subreddit(&mut self, sub: &str) -> Result<&mut Self, PSError> {
        if VALID_REDDIT.is_match(sub) {
            Ok(self.add_param("subreddit", sub)?)
        } else {
            Err(PSError::InvalidSubreddit(sub.to_string()))
        }
    }

    pub fn size(&mut self, size: u32) -> Result<&mut Self, PSError> {
        if size <= MAX_PS_FETCH_SIZE {
            Ok(self.add_param("size", &size.to_string())?)
        } else {
            Err(PSError::SizeTooHigh(size))
        }
    }

    fn add_param(&mut self, param: &str, par_options: &str) -> Result<&mut Self, PSError> {
        // Add_param() only allows a parameter to be added once. I decided against allowing
        // replacement in order to be as explicit as possible.
        if self.params.contains_key(param) {
            Err(PSError::AlreadyAdded(param.to_string()))
        } else {
            self.params
                .insert(param.to_string(), par_options.to_string());
            Ok(self)
        }
    }
}
