# triplet-miner-rs

A pure Rust port of [triplet-miner](https://github.com/SuperInstance/triplet-miner) — metric learning triplet mining with semi-hard mining, distance computation, and quality filtering.

## Features

- **Triplet data structure** — (anchor, positive, negative) with serialization
- **Mining strategies** — Random, Hard Negative, Semi-Hard, Domain-Aware
- **Text similarity** — Jaccard similarity, tokenization
- **Quality filtering** — Length checks, deduplication, scoring
- **Distance metrics** — Euclidean, cosine, Manhattan for embedding vectors
- **Evaluation** — Triplet quality metrics (separation, avg similarities)

## Usage

```rust
use triplet_miner::{Triplet, MiningStrategy, select_negatives, QualityFilter, distance};

let t = Triplet::new("fix bug in parser", "fix parsing error", "update readme");
let negs = select_negatives("fix bug", "fix issue", &["unrelated", "fix leak"], MiningStrategy::SemiHard, 2);

let d = distance::euclidean(&[1.0, 2.0], &[3.0, 4.0]);
```

## License

MIT
