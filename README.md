# Query Translator

Parse native query languages into an AST to enable generating queries in a different query language.

For example:
PromQL <--> AST <--> SPL

```
$ cargo run
Type '\q' to exit.
input> metric_name{test_one="one"}[5y]
parsed: promql
promql: metric_name{test_one="one"}[5y]
spl: index="metric_name" | test_one="one" | earliest=-5y
```