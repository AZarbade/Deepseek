# DeepSeek

A localised search engine


## References
1. Concept: [TF-IDF](https://en.wikipedia.org/wiki/Tf–idf)
2. Inspiration: [pdfgrep](https://pdfgrep.org/)
3. Primary library used: [poppler-rs](https://crates.io/crates/poppler-rs)

## Special thanks to,
[TSODING](https://www.youtube.com/tsoding) for being a chad dev.

## Stats

### Quering Perf

| Type     | Release | Debug |
|----------|---------|-------|
| no cache |  830ms  | 4.71s |
| df cache |  11ms   |  -ms  |
| tfn cache|  3ms   |  5ms  |


P.S. Don't look at the commit history; was experimentating.
