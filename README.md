# triplet-miner-rs

<<<<<<< HEAD
A pure Rust port of [triplet-miner](https://github.com/SuperInstance/triplet-miner) — metric learning triplet mining with semi-hard mining, distance computation, and quality filtering.

## Features

- **Triplet data structure** — (anchor, positive, negative) with serialization
- **Mining strategies** — Random, Hard Negative, Semi-Hard, Domain-Aware
- **Text similarity** — Jaccard similarity, tokenization
- **Quality filtering** — Length checks, deduplication, scoring
- **Distance metrics** — Euclidean, cosine, Manhattan for embedding vectors
- **Evaluation** — Triplet quality metrics (separation, avg similarities)
=======
Rust port of [triplet-miner](https://github.com/SuperInstance/triplet-miner) — triplet mining for contrastive learning.

## Features

- **Random mining**: random negative from different class
- **Hard mining**: closest negative (most violating)
- **Semi-hard mining**: closest negative that's farther than positive
- **Distance matrix**: precompute all pairwise distances
- **Loss metrics**: average triplet loss, violation rate
>>>>>>> 7b964d8 (Initial Rust port: triplet mining for contrastive learning)

## Usage

```rust
<<<<<<< HEAD
use triplet_miner::{Triplet, MiningStrategy, select_negatives, QualityFilter, distance};

let t = Triplet::new("fix bug in parser", "fix parsing error", "update readme");
let negs = select_negatives("fix bug", "fix issue", &["unrelated", "fix leak"], MiningStrategy::SemiHard, 2);

let d = distance::euclidean(&[1.0, 2.0], &[3.0, 4.0]);
=======
use triplet_miner::{mine_triplets, Strategy, average_loss, violation_rate};

let embeddings = vec![
    vec![0.0, 0.0], vec![0.5, 0.5],  // class 0
    vec![10.0, 10.0], vec![10.5, 10.5], // class 1
];
let labels = vec![0, 0, 1, 1];

let triplets = mine_triplets(&embeddings, &labels, Strategy::Hard, 100);
let loss = average_loss(&triplets, 1.0);
let rate = violation_rate(&triplets, 1.0);
>>>>>>> 7b964d8 (Initial Rust port: triplet mining for contrastive learning)
```

## License

MIT
