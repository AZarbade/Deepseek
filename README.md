# DeepSeek

A localised search engine


## References
1. [TF-IDF](https://en.wikipedia.org/wiki/Tf–idf)
2. [pdfgrep](https://pdfgrep.org/)
3. [poppler]()

## Stats
### Indexing Perf
   Release:

   no caching:
   ________________________________________________________
   Executed in   49.65 secs    fish           external
      usr time   27.32 secs    0.18 millis   27.32 secs
      sys time   19.17 secs    1.86 millis   19.17 secs

   full caching:
   ________________________________________________________
   Executed in   45.74 secs    fish           external
      usr time   27.86 secs  231.00 micros   27.86 secs
      sys time   14.00 secs  252.00 micros   14.00 secs

   Debug:

   no caching:
   ________________________________________________________
   Executed in   57.26 secs    fish           external
      usr time   35.85 secs    0.00 micros   35.85 secs
      sys time   17.57 secs  438.00 micros   17.57 secs

   full caching:
   ________________________________________________________
   Executed in   54.52 secs    fish           external
      usr time   36.81 secs   15.00 micros   36.81 secs
      sys time   13.87 secs  980.00 micros   13.87 secs

### Quering Perf

| Release  | Time  |
|----------|-------|
| no cache | 830ms |
| df cache | 11ms  |
| tfn cache| 3ms   |

| Debug    | Time  |
|----------|-------|
| no cache | 4.71s |
| df cache | --ms  |
| tfn cache| 5ms   |


P.S. Don't look at the commit history; was experimentating.
