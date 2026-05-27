//! Mining strategies for selecting negatives.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MiningStrategy {
    Random,
    HardNegative,
    SemiHard,
    DomainAware,
}

fn token_set(text: &str) -> std::collections::HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 2)
        .map(|w| w.to_lowercase())
        .collect()
}

/// Jaccard similarity between two texts.
pub fn jaccard(a: &str, b: &str) -> f64 {
    let sa = token_set(a);
    let sb = token_set(b);
    if sa.is_empty() && sb.is_empty() {
        return 0.0;
    }
    let inter = sa.intersection(&sb).count() as f64;
    let union = sa.union(&sb).count() as f64;
    if union == 0.0 {
        0.0
    } else {
        inter / union
    }
}

/// Compute text similarity (Jaccard-based).
pub fn compute_similarity(a: &str, b: &str) -> f64 {
    jaccard(a, b)
}

/// Select negative candidates according to the given strategy. Returns (candidate, similarity_to_anchor).
pub fn select_negatives(
    anchor: &str,
    positive: &str,
    candidates: &[String],
    strategy: MiningStrategy,
    n: usize,
) -> Vec<(String, f64)> {
    if candidates.is_empty() {
        return vec![];
    }
    let mut scored: Vec<(String, f64)> = candidates
        .iter()
        .map(|c| (c.clone(), compute_similarity(anchor, c)))
        .collect();

    match strategy {
        MiningStrategy::Random => {
            scored.truncate(n.min(scored.len()));
            scored
        }
        MiningStrategy::HardNegative => {
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            scored.truncate(n);
            scored
        }
        MiningStrategy::SemiHard => {
            let pos_sim = compute_similarity(anchor, positive);
            scored.retain(|(_, s)| *s > 0.0 && *s < pos_sim);
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            scored.truncate(n);
            scored
        }
        MiningStrategy::DomainAware => {
            scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            scored.truncate(n);
            scored
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jaccard_same() {
        assert!((jaccard("hello world", "hello world") - 1.0).abs() < 0.01);
    }
    #[test]
    fn test_jaccard_disjoint() {
        assert!(jaccard("aaa bbb", "ccc ddd") < 0.01);
    }
    #[test]
    fn test_hard() {
        let cands: Vec<String> = vec![
            "unrelated thing".into(),
            "fix bug leak".into(),
            "hello there".into(),
        ];
        let r = select_negatives(
            "fix bug",
            "fix issue",
            &cands,
            MiningStrategy::HardNegative,
            2,
        );
        assert_eq!(r.len(), 2);
    }
}
