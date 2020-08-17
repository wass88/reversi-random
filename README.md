# Reversi Random Ai

## Protocol

矢印は入出力を示すだけで，実際に入出力する必要はない．

`<- stdin`, `-> stdout`,
stderr は無視される (見えるようにする予定)

Example:

```
<- init 1
<- played put 4 5
<- wait
-> play put 5 5
...
<- result 0
```

- <- `init (0|1)`: 0 は先手 1 は後手
- <- `played (put \d \d| pass)`: 相手の手
- <- `wait`: 入力待ち
- -> `(put \d \d| pass)`: 手
- <- `result \d+`: あなたの結果 (-64 ~ 64)
