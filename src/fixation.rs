use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref FIXATION_BOUNDARY_LIST: Vec<Vec<u8>> = vec![
        vec![0, 4, 12, 17, 24, 29, 35, 42, 48],
        vec![1, 2, 7, 10, 13, 14, 19, 22, 25, 28, 31, 34, 37, 40, 43, 46, 49,],
        vec![
            1, 2, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45,
            47, 49,
        ],
        vec![
            0, 2, 4, 5, 6, 8, 9, 11, 14, 15, 17, 18, 20, 0, 21, 23, 24, 26, 27, 29, 30, 32, 33, 35,
            36, 38, 39, 41, 42, 44, 45, 47, 48,
        ],
        vec![
            0, 2, 3, 5, 6, 7, 8, 10, 11, 12, 14, 15, 17, 19, 20, 21, 23, 24, 25, 26, 28, 29, 30,
            32, 33, 34, 35, 37, 38, 39, 41, 42, 43, 44, 46, 47, 48,
        ],
    ];
    static ref FIXATION_CACHE: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn get_fixation(word: &str, fixation_point: usize) -> usize {
    match FIXATION_CACHE.lock() {
        Ok(mut cache) => match cache.get(word) {
            None => {
                let fixation = calculate_fixation(word, fixation_point);
                cache.insert(word.to_string(), fixation);
                fixation
            }
            Some(fixation) => *fixation,
        },
        Err(_) => calculate_fixation(word, fixation_point),
    }
}

pub fn calculate_fixation(word: &str, fixation_point: usize) -> usize {
    let word_length = word.chars().collect::<Vec<_>>().len();
    let boundary = match FIXATION_BOUNDARY_LIST.get(fixation_point) {
        None => FIXATION_BOUNDARY_LIST.get(0).unwrap(),
        Some(list) => list,
    };

    let fixation_length_from_last = boundary
        .iter()
        .position(|&index| word_length <= index as usize)
        .unwrap_or(word_length.saturating_sub(boundary.len()));

    let fixation_length = word_length.saturating_sub(fixation_length_from_last);

    fixation_length.max(0)
}

#[cfg(test)]
mod test {
    use super::calculate_fixation;

    macro_rules! fixation_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (input, expected) = $value;
                    assert_eq!(expected, calculate_fixation(input, 0));
                }
            )*
        }
    }

    fixation_tests! {
        fixation_empty: ("", 0),
        fixation_test: ("test", 3),
        fixation_is: ("is", 1),
        fixation_bionic: ("Bionic", 4),
        fixation_reading: ("Reading", 5),
    }
}
