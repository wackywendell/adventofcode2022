# README

Run day `$n` with `cargo run -- $n`.

Generated files from day 7 on with:

```sh
for i in `seq 07 25`; do nn=$(printf %02d $i); echo $nn; touch inputs/day$nn.txt; cat src/problems/template.rs | sed "s/00/$nn/" > src/problems/day$nn.rs;  done
```