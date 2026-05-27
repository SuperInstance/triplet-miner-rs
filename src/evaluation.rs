//! Evaluation metrics for triplet quality.

/// Metrics computed over a set of triplets.
#[derive(Debug, Clone)]
pub struct TripletMetrics {
    pub count: usize,
    pub avg_anchor_positive_sim: f64,
    pub avg_anchor_negative_sim: f64,
    pub separation: f64,
}

/// Evaluate triplet quality from similarity scores.
pub fn evaluate_triplets(ap_sims: &[f64], an_sims: &[f64]) -> TripletMetrics {
    let n = ap_sims.len().min(an_sims.len());
    if n == 0 {
        return TripletMetrics {
            count: 0,
            avg_anchor_positive_sim: 0.0,
            avg_anchor_negative_sim: 0.0,
            separation: 0.0,
        };
    }
    let avg_ap: f64 = ap_sims.iter().take(n).sum::<f64>() / n as f64;
    let avg_an: f64 = an_sims.iter().take(n).sum::<f64>() / n as f64;
    TripletMetrics {
        count: n,
        avg_anchor_positive_sim: avg_ap,
        avg_anchor_negative_sim: avg_an,
        separation: avg_ap - avg_an,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let m = evaluate_triplets(&[0.8, 0.9], &[0.1, 0.2]);
        assert_eq!(m.count, 2);
        assert!(m.separation > 0.5);
    }
}
