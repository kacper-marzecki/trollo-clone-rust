
pub fn is_blank(it: &String) -> bool {
    return it.is_empty()
        || is_whitespace(it)
}

pub fn is_whitespace(it: &String) -> bool {
    it.trim().is_empty()
}