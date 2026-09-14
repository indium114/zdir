# zdir

_zdir_ is a blazingly-faster `cd` alternative, inspired by [zoxide](https://github.com/ajeetdsouza/zoxide).

## benchmarks

> as of commit `212325d1`, using nushell 0.115.1

```
==== zoxide benchmark
╭──────┬──────────────────╮
│ mean │ 8ms 798µs 619ns  │
│ min  │ 5ms 768µs 335ns  │
│ max  │ 18ms 768µs 812ns │
│ std  │ 1ms 826µs 13ns   │
╰──────┴──────────────────╯
==== zdir benchmark
╭──────┬──────────────────╮
│ mean │ 4ms 762µs 404ns  │
│ min  │ 3ms 246µs 124ns  │
│ max  │ 12ms 194µs 279ns │
│ std  │ 1ms 649µs 303ns  │
╰──────┴──────────────────╯
```
