mod dist;

pub fn get(code: impl Into<String>) -> Option<Vec<u8>> {
    let code = code.into();
    let code = format!("{:x}", code.chars().next()? as u32);
    dist::get(code)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_find() {
        assert!(get("🤔").is_some());
        assert!(get("a").is_none());
    }
    #[test]
    fn performance() {
        let mut count = 0;
        for _ in 0..1000 {
            if get("🤔").is_some() {
                count += 1;
            }
        }
        assert_eq!(count, 1000);
    }
}
