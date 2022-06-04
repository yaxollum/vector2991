# vector2991

A calculator that can do operations on 2D algebraic vectors (addition, subtraction, scalar multiplication). For both input and output, vectors can be represented in three forms:
1. Coordinates (e.g. `(30.31,-17.50)`)
2. True Bearing (e.g. `35 [120]`)
3. Quadrant Bearing (e.g. `35 [S60W]`)

## Example session

```
> 35 [N120E]
(30.31,-17.50)
> 35 [N120E]_q
35.00 [S60W]
> 35[N12E] + 5[S5W]
(6.84,29.25)
> 35[N12E] + 5[S5W]_t
30.04 [13.16]
> 35[N12E] + 5[S5W]_q
30.04 [N13E]
> 3*35[N]-2*5[S]
(0.00,115.00)
> 3*5*5[E]
(75.00,0.00)
> 3*5*5[E]_q
75.00 [E]
```

## Grammar
vector2991's LALRPOP grammar can be found at [src/parser.lalrpop](https://github.com/yaxollum/vector2991/blob/main/src/parser.lalrpop).
