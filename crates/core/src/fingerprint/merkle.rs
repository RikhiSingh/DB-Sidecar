use sha2::{Digest, Sha256};

pub fn merkle_root(mut leaves: Vec<(String, String)>) -> String {
    // leaves: (qualified_name, hash)
    leaves.sort_by(|a, b| a.0.cmp(&b.0));
    let mut level: Vec<String> = leaves
        .into_iter()
        .map(|(_, h)| h)
        .collect();
    if level.is_empty() { return String::new(); }
    while level.len() > 1 {
        let mut next = Vec::with_capacity((level.len() + 1) / 2);
        for chunk in level.chunks(2) {
            let pair = if chunk.len() == 2 { format!("{}{}", chunk[0], chunk[1]) } else { format!("{}{}", chunk[0], chunk[0]) };
            let mut hasher = Sha256::new();
            hasher.update(pair.as_bytes());
            next.push(format!("{:x}", hasher.finalize()));
        }
        level = next;
    }
    level.remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_changes() {
        let r1 = merkle_root(vec![
            ("public.a".to_string(), "aa".to_string()),
            ("public.b".to_string(), "bb".to_string()),
        ]);
        let r2 = merkle_root(vec![
            ("public.a".to_string(), "aa".to_string()),
            ("public.b".to_string(), "bc".to_string()),
        ]);
        assert_ne!(r1, r2);
    }
}


