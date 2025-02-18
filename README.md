# elastic_hash_rs
 Port of [MWARDUNI/ElasticHashing](https://github.com/MWARDUNI/ElasticHashing) to Rust

Based on "Optimal Bounds for Open Addressing Without Reordering - Martin Farach-Colton, Andrew Krapivin, William Kuszmaul" which is readable https://arxiv.org/pdf/2501.02305 and was written about https://www.quantamagazine.org/undergraduate-upends-a-40-year-old-data-science-conjecture-20250210/.

This code is minimal and not *yet* optimized.
Ideas for improvement:
 * an actually good API, like hashbrown
 * no-std support with a pluggable Allocator
 * performance (SIMD??????????)

#### References:
 * [Another python impl](https://github.com/sternma/optopenhash/blob/main/optopenhash/elastic_hashing.py)
 * [HackerNews Discussion](https://news.ycombinator.com/item?id=43002511)
 * [Lobsters Discussion](https://lobste.rs/s/uoiosa/undergraduate_invents_faster_hash_table)
