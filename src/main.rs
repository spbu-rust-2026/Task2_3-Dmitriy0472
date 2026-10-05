fn main() {
    let mut text = String::from("Пик Балмера");
    reverse_words_at_odd_positions(&mut text);
    println!("{text}");
}

fn reverse_words_at_odd_positions(text: &mut String) {
    let n = text.len();
    let mut i = 0;
    let mut word_number = 0;

    while i < n {
        let c = text[i..].chars().next().unwrap();
        if c.is_whitespace() {
            i += c.len_utf8();
            continue;
        }

        let start = i;

        while i < n {
            let c = text[i..].chars().next().unwrap();
            if c.is_whitespace() {
                break;
            }
            i += c.len_utf8();
        }

        word_number += 1;
        if word_number % 2 == 1 {
            let word = &mut unsafe { text.as_bytes_mut() }[start..i];

            word.reverse();

            let mut j = 0;
            while j < word.len() {
                let mut k = j;
                while word[k] & 0xC0 == 0x80 {
                    k += 1;
                }
                word[j..=k].reverse();
                j = k + 1;
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/monologue.rs"]
mod monologue_tests;
