use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref CONVERTIBLE_REGEX: Regex = Regex::new(r"(<!--[\s\S]*?-->)|(<[^>]*>)").unwrap();
}

pub fn build_is_html_checker(text: &str) -> impl Fn(Match) -> bool {
    let html_tag_iter = CONVERTIBLE_REGEX.find_iter(text).collect::<Vec<_>>();
    let html_tag_range_iter: Vec<_> = html_tag_iter
        .iter()
        .map(|x| (x.start(), x.start() + x.as_str().len() - 1))
        .collect();
    let reversed_html_tag_iter: Vec<_> = html_tag_range_iter.into_iter().rev().collect();

    move |m| {
        let start_index = m.start();
        let tag_range = reversed_html_tag_iter
            .iter()
            .find(|(range_start, _)| start_index > *range_start);

        if let Some((_, range_end)) = tag_range {
            let is_include = start_index < *range_end;
            return is_include;
        }

        false
    }
}
