use rand::{distributions::Alphanumeric, Rng};

pub fn gen_str(len: usize) -> String {
    let str = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>();

    hex::encode(str)[..len].to_string()
}

pub fn get_val(text: &str, key: &str) -> String {
    let split = text
        .split_whitespace()
        .collect::<Vec<&str>>();

    let find = format!("/{}", key);

    for (k, v) in split.iter().enumerate() {
        if *v != find {
            continue;
        }

        if k + 1 > split.len() {
            break;
        }

        let next = split[k + 1];
        if next.starts_with('/') {
            break;
        }

        return next.to_string();
    }

    String::new()
}

pub fn get_flag(text: &str, key: &str) -> bool {
    let split = text
        .split_whitespace()
        .collect::<Vec<&str>>();

    let find = format!("/{}", key);

    for token in split {
        if token == find {
            return true;
        }
    }

    false
}