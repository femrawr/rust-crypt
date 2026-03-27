use rand::RngExt;
use rand::distr::Alphanumeric;

pub fn gen_str(len: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>()
}

pub fn get_val(list: &Vec<String>, key: &str) -> String {
    list
        .iter()
        .position(|str| str == &format!("/{}", key))
        .and_then(|i| list.get(i + 1))
        .filter(|next| !next.starts_with('/'))
        .map(|str| str.to_string())
        .unwrap_or_default()
}

pub fn has_val(list: &Vec<String>, key: &str) -> bool {
    list
        .iter()
        .any(|str| str == &format!("/{}", key))
}
