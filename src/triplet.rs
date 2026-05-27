//! Triplet data structure for contrastive learning.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A (anchor, positive, negative) triplet for contrastive learning.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Triplet {
    pub anchor: String,
    pub positive: String,
    pub negative: String,
    pub similarity: f64,
    pub source: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Triplet {
    pub fn new(
        anchor: impl Into<String>,
        positive: impl Into<String>,
        negative: impl Into<String>,
    ) -> Self {
        Self {
            anchor: anchor.into(),
            positive: positive.into(),
            negative: negative.into(),
            similarity: 0.0,
            source: String::new(),
            metadata: HashMap::new(),
        }
    }
    pub fn with_similarity(mut self, s: f64) -> Self {
        self.similarity = s;
        self
    }
    pub fn with_source(mut self, s: impl Into<String>) -> Self {
        self.source = s.into();
        self
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    pub fn from_json(s: &str) -> Option<Self> {
        serde_json::from_str(s).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_roundtrip() {
        let t = Triplet::new("hello", "world", "foo").with_similarity(0.5);
        let json = t.to_json();
        let t2 = Triplet::from_json(&json).unwrap();
        assert_eq!(t.anchor, t2.anchor);
        assert_eq!(t.similarity, t2.similarity);
    }
}
