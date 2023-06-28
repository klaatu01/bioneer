//! # `bioneer` - Bionic Reading Library
//!
//! The `bioneer` crate is a bionic reading library for Rust that converts text into a bionic reading format.
//! This library enhances the readability and comprehension of text by applying formatting techniques.
//! This crate is a Rust port of the JavaScript library [text-vide](https://github.com/Gumball12/text-vide).
//!
//! ## Usage
//!
//! To use the `bioneer` library, add the following dependency to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! bioneer = "0.1.1"
//! ```
//!
//! Then, import the necessary items into your Rust code:
//!
//! ```rust
//! use bioneer::Bionify;
//! ```
//!
//! ## Bionify Trait
//!
//! The `Bionify` trait provides a method for bionifying strings.
//!
//! ### Required Method
//!
//! ```rust
//! use bioneer::Bionify;
//!
//! let text = "Hello, world!";
//! let bionified_text = text.bionify();
//! println!("{}", bionified_text);
//! // "<b>Hel</b>lo, <b>Wor</b>ld!"
//! ```
//!
//! ### Examples
//!
//! Bionify can be used with both `String` and `&str` types:
//!
//! ```rust
//! use bioneer::Bionify;
//!
//! let text = "Hello, world!";
//! let bionified_text = text.bionify();
//! println!("{}", bionified_text);
//! // "<b>Hel</b>lo, <b>Wor</b>ld!"
//!
//! let owned_text = String::from("Hello, world!");
//! let owned_bionified_text = owned_text.bionify();
//! println!("{}", owned_bionified_text);
//! // "<b>Hel</b>lo, <b>Wor</b>ld!"
//! ```

mod fixation;
mod highlight;
mod html_tag;

use fixation::get_fixation;
use highlight::highlight;
use html_tag::build_is_html_checker;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CONVERTIBLE_REGEX: Regex =
        Regex::new(r"(\p{L}|\p{Nd})*\p{L}(\p{L}|\p{Nd})*").unwrap();
}

/// The `Bionify` trait provides a method for bionifying strings.
pub trait Bionify {
    /// Converts the implementing string into a bionic reading format.
    fn bionify(&self) -> String;
}

impl Bionify for String {
    fn bionify(&self) -> String {
        bionify(self)
    }
}

impl Bionify for str {
    fn bionify(&self) -> String {
        bionify(self)
    }
}

pub(crate) fn bionify(text: &str) -> String {
    let matches = CONVERTIBLE_REGEX.find_iter(text);

    let mut result: String = String::new();
    let mut last_match_index = 0;

    let is_html = build_is_html_checker(text);

    for m in matches {
        if is_html(m) {
            continue;
        }
        let start_index = m.start();
        let fixation = get_fixation(m.as_str(), 0);
        let substring_iter = m.as_str().chars().take(fixation);
        let end_index = start_index + fixation;
        let offset = substring_iter.clone().collect::<String>().len() - substring_iter.count();

        result.push_str(&text[last_match_index..start_index]);
        if start_index != end_index + offset {
            let highlighted = &highlight(&text[start_index..end_index + offset]);
            result.push_str(highlighted)
        }

        last_match_index = end_index + offset;
    }

    result.push_str(&text[last_match_index..]);
    result
}

#[cfg(test)]
mod test {
    use crate::Bionify;

    #[test]
    fn hello_world() {
        let text = "Hello, World!";
        let expected_output = "<b>Hel</b>lo, <b>Wor</b>ld!";

        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn english_paragraph_1() {
        let text =
      "orem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let expected_output =
      "<b>ore</b>m <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t, <b>consetet</b>ur <b>sadipsci</b>ng <b>eli</b>tr, <b>se</b>d <b>dia</b>m <b>nonu</b>my <b>eirm</b>od <b>temp</b>or <b>invidu</b>nt <b>u</b>t <b>labo</b>re <b>e</b>t <b>dolo</b>re <b>mag</b>na <b>aliquy</b>am <b>era</b>t, <b>se</b>d <b>dia</b>m <b>volupt</b>ua. <b>A</b>t <b>ver</b>o <b>eo</b>s <b>e</b>t <b>accus</b>am <b>e</b>t <b>jus</b>to <b>du</b>o <b>dolor</b>es <b>e</b>t <b>e</b>a <b>reb</b>um. <b>Ste</b>t <b>cli</b>ta <b>kas</b>d <b>gubergr</b>en, <b>n</b>o <b>se</b>a <b>takima</b>ta <b>sanct</b>us <b>es</b>t <b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t. <b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t, <b>consetet</b>ur <b>sadipsci</b>ng <b>eli</b>tr, <b>se</b>d <b>dia</b>m <b>nonu</b>my <b>eirm</b>od <b>temp</b>or <b>invidu</b>nt <b>u</b>t <b>labo</b>re <b>e</b>t <b>dolo</b>re <b>mag</b>na <b>aliquy</b>am <b>era</b>t, <b>se</b>d <b>dia</b>m <b>volupt</b>ua. <b>A</b>t <b>ver</b>o <b>eo</b>s <b>e</b>t <b>accus</b>am <b>e</b>t <b>jus</b>to <b>du</b>o <b>dolor</b>es <b>e</b>t <b>e</b>a <b>reb</b>um. <b>Ste</b>t <b>cli</b>ta <b>kas</b>d <b>gubergr</b>en, <b>n</b>o <b>se</b>a <b>takima</b>ta <b>sanct</b>us <b>es</b>t <b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t.";

        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn english_paragraph_2() {
        let text =
      "Bionic Reading is a new method facilitating the reading process by guiding the eyes through text with artificial fixation points. As a result, the reader is only focusing on the highlighted initial letters and lets the brain center complete the word. In a digital world dominated by shallow forms of reading, Bionic Reading aims to encourage a more in-depth reading and understanding of written content.";
        let expected_output =
      "<b>Bion</b>ic <b>Readi</b>ng <b>i</b>s a <b>ne</b>w <b>meth</b>od <b>facilitati</b>ng <b>th</b>e <b>readi</b>ng <b>proce</b>ss <b>b</b>y <b>guidi</b>ng <b>th</b>e <b>eye</b>s <b>throu</b>gh <b>tex</b>t <b>wit</b>h <b>artifici</b>al <b>fixati</b>on <b>poin</b>ts. <b>A</b>s a <b>resu</b>lt, <b>th</b>e <b>read</b>er <b>i</b>s <b>onl</b>y <b>focusi</b>ng <b>o</b>n <b>th</b>e <b>highlight</b>ed <b>initi</b>al <b>lette</b>rs <b>an</b>d <b>let</b>s <b>th</b>e <b>bra</b>in <b>cent</b>er <b>comple</b>te <b>th</b>e <b>wor</b>d. <b>I</b>n a <b>digit</b>al <b>wor</b>ld <b>dominat</b>ed <b>b</b>y <b>shall</b>ow <b>for</b>ms <b>o</b>f <b>readi</b>ng, <b>Bion</b>ic <b>Readi</b>ng <b>aim</b>s <b>t</b>o <b>encoura</b>ge a <b>mor</b>e <b>i</b>n-<b>dep</b>th <b>readi</b>ng <b>an</b>d <b>understand</b>ing <b>o</b>f <b>writt</b>en <b>conte</b>nt.";

        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn utf_8_paragraph() {
        let text =
      "Bionic Réading is a néw method facilitating the réading process by guiding the eyes through text with artificial fixation points. As a result, the reader is only focusing on the highlighted initial letters and lets the brain center complete the word. In a digital world dominated by shallow forms of reading, Bionic Reading aims to encourage a more in-depth reading and understanding of written content.";
        let expected_output =
      "<b>Bion</b>ic <b>Réadi</b>ng <b>i</b>s a <b>né</b>w <b>meth</b>od <b>facilitati</b>ng <b>th</b>e <b>réadi</b>ng <b>proce</b>ss <b>b</b>y <b>guidi</b>ng <b>th</b>e <b>eye</b>s <b>throu</b>gh <b>tex</b>t <b>wit</b>h <b>artifici</b>al <b>fixati</b>on <b>poin</b>ts. <b>A</b>s a <b>resu</b>lt, <b>th</b>e <b>read</b>er <b>i</b>s <b>onl</b>y <b>focusi</b>ng <b>o</b>n <b>th</b>e <b>highlight</b>ed <b>initi</b>al <b>lette</b>rs <b>an</b>d <b>let</b>s <b>th</b>e <b>bra</b>in <b>cent</b>er <b>comple</b>te <b>th</b>e <b>wor</b>d. <b>I</b>n a <b>digit</b>al <b>wor</b>ld <b>dominat</b>ed <b>b</b>y <b>shall</b>ow <b>for</b>ms <b>o</b>f <b>readi</b>ng, <b>Bion</b>ic <b>Readi</b>ng <b>aim</b>s <b>t</b>o <b>encoura</b>ge a <b>mor</b>e <b>i</b>n-<b>dep</b>th <b>readi</b>ng <b>an</b>d <b>understand</b>ing <b>o</b>f <b>writt</b>en <b>conte</b>nt.";

        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn very_long_word() {
        let text =
      "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let expected_output =
      "<b>aaaaaaaaa</b>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn with_html() {
        let text = "<a>abcd</a>efg";
        let expected_output = "<a><b>abc</b>d</a><b>ef</b>g";
        let output = text.bionify();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn complex_html() {
        let text = r#"<div class="bionic-reader-container">
            
            
    <span class="w bionic"><b class="b bionic">nor</b>mal </span><span class="w bionic"><b class="b bionic">te</b>xt</span>: <span class="w bionic"><b class="b bionic">abcd</b>efg</span><br><span class="w bionic"><b class="b bionic">wi</b>th </span><span class="w bionic"><b class="b bionic">a</b> </span><span class="w bionic"><b class="b bionic">t</b>ag</span>: <a target="_blank"><span class="w bionic"><b class="b bionic">ab</b>cd</span></a><span class="w bionic"><b class="b bionic">e</b>fg</span><br><span class="w bionic"><b class="b bionic">wi</b>th </span><span class="w bionic"><b class="b bionic">b</b> </span><span class="w bionic"><b class="b bionic">t</b>ag</span>: <b><span class="w bionic"><b class="b bionic">ab</b>cd</span></b><span class="w bionic"><b class="b bionic">e</b>fg</span><br><span class="w bionic"><b class="b bionic">wi</b>th </span><span class="w bionic"><b class="b bionic">d</b>iv </span><span class="w bionic"><b class="b bionic">t</b>ag</span>: <div><span class="w bionic"><b class="b bionic">ab</b>cd</span></div><span class="w bionic"><b class="b bionic">e</b>fg</span><br>
    
        <!-- <div class="br-foot-node">
            <p style="margin: 32px 0 32px 70px; font-weight: 700; font-size: 26px; line-height: 1.6em;">
                —
            </p>
            <p>
                Bionic Reading<sup>®</sup><br>
                A higher dimension of reading.<br>
                <a href="https://bionic-reading.com">bionic-reading.com</a>
            </p>
            <br/>
            <br/>
            <p>
                
            </p>
        </div> -->
    
</div>"#;

        let expected = r#"<div class="bionic-reader-container">
            
            
    <span class="w bionic"><b class="b bionic"><b>no</b>r</b><b>ma</b>l </span><span class="w bionic"><b class="b bionic"><b>t</b>e</b><b>x</b>t</span>: <span class="w bionic"><b class="b bionic"><b>abc</b>d</b><b>ef</b>g</span><br><span class="w bionic"><b class="b bionic"><b>w</b>i</b><b>t</b>h </span><span class="w bionic"><b class="b bionic">a</b> </span><span class="w bionic"><b class="b bionic">t</b><b>a</b>g</span>: <a target="_blank"><span class="w bionic"><b class="b bionic"><b>a</b>b</b><b>c</b>d</span></a><span class="w bionic"><b class="b bionic">e</b><b>f</b>g</span><br><span class="w bionic"><b class="b bionic"><b>w</b>i</b><b>t</b>h </span><span class="w bionic"><b class="b bionic">b</b> </span><span class="w bionic"><b class="b bionic">t</b><b>a</b>g</span>: <b><span class="w bionic"><b class="b bionic"><b>a</b>b</b><b>c</b>d</span></b><span class="w bionic"><b class="b bionic">e</b><b>f</b>g</span><br><span class="w bionic"><b class="b bionic"><b>w</b>i</b><b>t</b>h </span><span class="w bionic"><b class="b bionic">d</b><b>i</b>v </span><span class="w bionic"><b class="b bionic">t</b><b>a</b>g</span>: <div><span class="w bionic"><b class="b bionic"><b>a</b>b</b><b>c</b>d</span></div><span class="w bionic"><b class="b bionic">e</b><b>f</b>g</span><br>
    
        <!-- <div class="br-foot-node">
            <p style="margin: 32px 0 32px 70px; font-weight: 700; font-size: 26px; line-height: 1.6em;">
                —
            </p>
            <p>
                Bionic Reading<sup>®</sup><br>
                A higher dimension of reading.<br>
                <a href="https://bionic-reading.com">bionic-reading.com</a>
            </p>
            <br/>
            <br/>
            <p>
                
            </p>
        </div> -->
    
</div>"#;
        let output = text.bionify();
        assert_eq!(output, expected);
    }
}
