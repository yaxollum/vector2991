# vector2991

vector2991 is a vector calculator that can do addition, subtraction, and scalar multiplication on 2D algebraic vectors. For both input and output, vector2991 supports three different vector representations:

1. Coordinates (e.g. `(30.31, -17.50)`)
2. True Bearing (e.g. `35 [120]`)
3. Quadrant Bearing (e.g. `35 [S60W]`)

## Build instructions

Simply run:

```
git clone https://github.com/yaxollum/vector2991
cd vector2991
cargo build --release
```

## Tutorial

In the directory where you built vector2991 (see above section), use the command

```
cargo run --release
```

to run the program.

You should see a vector2991 shell where you can perform various operations on vectors.

### Coordinates

An algebraic vector can be represented using coordinates in the form `(x,y)`. If you try entering `(2,3)` in the vector2991 shell, you should get the output `(2.00,3.00)`.

### Vector arithmetic

vector2991 supports three vector arithmetic operations: addition, subtraction, and scalar multiplication. Some examples:

1. Addition

```
> (2,3)+(3,4)
(5.00,7.00)
```

2. Subtraction

```
> (2,3)-(3,4)
(-1.00,-1.00)
```

3. Scalar multiplication

```
> 5*(2,3)
(10.00,15.00)
```

4. Combining the operations

```
> (2,3)-(3,4)+5*(1,1)
(4.00,4.00)
```

### Order of operations

By default, scalar multiplication is evaluated first, followed by addition and subtraction being evaluated from left to right. To change this order, you can use brackets to group expressions:

```
> (2,3)-((3,4)+5*(1,1))
(-6.00,-6.00)
> 5*((1,1)+(2,2))
(15.00,15.00)
```

### Other vector representations

In addition to coordinate vectors, vector2991 also supports vectors represented using [true bearing and quadrant bearing](http://academic.brooklyn.cuny.edu/geology/leveson/core/linksa/comp.html). For the compass directions, vector2991 uses the positive y-axis as north and the positive x-axis as east.

#### Input

To enter a vector in true bearing, specify the magnitude of the vector and - in square brackets - the clockwise angle between the vector and north. For example, `53 [120]` would mean a vector with a magnitude of `53` that has a direction 120 degrees clockwise from north.

To enter a vector in quadrant bearing, specify the magnitude of the vector and - in square brackets - its direction in terms of north, south, east, and west. For example, `53 [W30S]` (west 30 degrees south) and `53 [S60W]` (south 60 degrees west) are both equivalent to `53 [120]`. vector2991's "quadrant bearing" also includes the cardinal directions and the ordinal directions - vectors like `53 [N]` and `53 [SE]` are valid syntax.

#### Output

By default, vector2991 represents all vectors using coordinates. No matter what vector format you use for input, the default output will always in the coordinate format:

```
> 53[N]+42[E]
(42.00,53.00)
> 5[NE]+6*(5,5)-2[S60E]
(31.80,34.54)
```

You can change the output format of an expression to true bearing or quadrant bearing by adding `_t` or `_q`, respectively, to the end of the line (no brackets required):

```
> 53[N]+42[E]_t
67.62 [38.40]
> 5[NE]+6*(5,5)-2[S60E]_q
46.95 [N43E]
```

### Example session

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
> 5[45]+3[N]
(3.54,6.54)
> 5*(3*(3[N]+4[90])+1[N])
(60.00,50.00)
```

## Grammar

vector2991's LALRPOP grammar can be found at [src/parser.lalrpop](https://github.com/yaxollum/vector2991/blob/main/src/parser.lalrpop).
