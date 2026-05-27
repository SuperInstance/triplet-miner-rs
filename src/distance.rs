//! Distance computation for embeddings.

/// Euclidean distance between two vectors.
pub fn euclidean(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}

/// Manhattan distance.
pub fn manhattan(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_euclidean() {
        assert!((euclidean(&[0.0, 0.0], &[3.0, 4.0]) - 5.0).abs() < 0.001);
    }
    #[test]
    fn test_cosine() {
        assert!((cosine_similarity(&[1.0, 0.0], &[0.0, 1.0])).abs() < 0.001);
    }
    #[test]
    fn test_cosine_same() {
        assert!((cosine_similarity(&[1.0, 2.0], &[1.0, 2.0]) - 1.0).abs() < 0.001);
    }
}
