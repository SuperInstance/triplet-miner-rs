//! Quality filters for mined triplets.

use crate::triplet::Triplet;
use std::collections::HashSet;

/// Filter and score triplets by quality criteria.
pub struct QualityFilter {
    pub min_length: usize,
    pub max_length: usize,
    pub deduplicate: bool,
    pub min_quality: f64,
}

impl Default for QualityFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl QualityFilter {
    pub fn new() -> Self {
        Self {
            min_length: 10,
            max_length: 50000,
            deduplicate: true,
            min_quality: 0.0,
        }
    }

    /// Compute a quality score (0.0 – 1.0) for a triplet.
    pub fn score(&self, t: &Triplet) -> f64 {
        let mut s = 0.5;
        for text in [&t.anchor, &t.positive] {
            let len = text.len();
            if len < self.min_length || len > self.max_length {
                return 0.0;
            }
            if (50..=2000).contains(&len) {
                s += 0.1;
            } else if (20..=5000).contains(&len) {
                s += 0.05;
            }
        }
        if t.negative.len() < self.min_length {
            return 0.0;
        }
        s += (t.similarity * 0.2).min(0.2);
        if !t.metadata.is_empty() {
            s += 0.05;
        }
        s.min(1.0)
    }

    /// Apply quality filters to a list of triplets.
    pub fn filter(&self, triplets: &[Triplet]) -> Vec<Triplet> {
        let mut result = Vec::new();
        let mut seen = HashSet::new();
        for t in triplets {
            if t.anchor.len() < self.min_length || t.positive.len() < self.min_length {
                continue;
            }
            if t.anchor.len() > self.max_length || t.positive.len() > self.max_length {
                continue;
            }
            let q = self.score(t);
            if q < self.min_quality {
                continue;
            }
            if self.deduplicate {
                let h = format!("{:x}", md5_hex(&format!("{}{}", t.anchor, t.positive)));
                if seen.contains(&h) {
                    continue;
                }
                seen.insert(h);
            }
            result.push(t.clone());
        }
        result
    }
}

fn md5_hex(s: &str) -> u64 {
    // Simple hash for dedup
    let mut h: u64 = 5381;
    for b in s.bytes() {
        h = h.wrapping_mul(33).wrapping_add(b as u64);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_filter() {
        let f = QualityFilter::new();
        let t = Triplet::new("a".repeat(50), "b".repeat(50), "c".repeat(50));
        assert!(f.score(&t) > 0.0);
    }
}
