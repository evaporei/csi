# Binary Representations of Data

## 1. Hexadecimal

### 1.1. Simple conversion

```
9 = 0x09
136 = 0x88 (128 + 8)
247 = 0xf7 (0xff - 0x08)
```

### 1.2. CSS colors

rgb(255, 0, 0) = 0xff0000

```
256 * 256 * 256
16.777.216 colors
```

hellohex

```
1 byte = 2 hex digits
17 bytes = 34 hex digits
68 65 6c 6c 6f 20 77 6f 72 6c 64 20 f0 9f 98 80 0a
```

first five bytes to binary:

```
68 65 6c 6c 6f
5 = 0b0101
6 = 0b0110
8 = 0b1000
c = 0b1100
f = 0b1111

68 = 0110 1000
65 = 0110 0101
6c = 0110 1100
6c = 0110 1100
6f = 0110 1111
```

## 2. Integers

### 2.1. Basic conversion

decimal to binary

```
4 = 0b0100
65 = 0b0100 0001
105 = 0b0110 1001
255 = 0b1111 1111
```

binary to decimal

```
10 = 2
11 = 3
1101100 = 108
1010101 = 85
```

### 2.2. Unsigned binary addition

```
 11111111 = 255
+00001101 =  13
---------
100001100 = 268
```

### 2.3. Two’s complement conversion

decimal do binary:

algo:

1. start w/ positive number
2. flip all bits
3. add one (ignoring any overflow)

```
127 = 0b0111 1111
-128 = 0b1000 0000 -> 0b0111 1111 -> 0b1000 0000
-1 = 0b0000 0001 -> 0b1111 1110 -> 0b1111 1111
1 = 0b0000 0001
-14 = 0b0000 1110 -> 0b1111 0001 -> 0b1111 0010
```

binary to decimal:

algo:

- 0b1000 0000 is the highest negative number (-128)
- 0b1111 1111 is the lowest negative number (-1)

-128 + 1 = -127 = 0b1000 0001 (counting backwards)

eg: 0b1000 0011 = 0b1000 0000 (-128) + 0b0000 0011 (3)

1. go from highest negative number (0b1000 0000)
2. then add the positive number that is "inside" the number (0b0000 0011)

```
10000011 -> -125
11000100 -> -60
```

### 2.4. Addition of two’s complement signed integers

```
01111111   (127)
10000000+  (-128)
11111111   (-1)
```

### 2.5. Advanced: Integer overflow detection

XOR(carry in, carry out) -> aka last two carried bits (last leftmost bit, and one after that, overflow)

## 3. Byte ordering

### 3.1. It’s over 9000!

```
00100011 00101001
35 41
35 * 256 + 41 = 9001
big endian
```

### 3.2. TCP

`tcpheader` file

```
sequence number = 441e 7368 = 1142846312
acknowledgment number = eff2 a002 = 4025655298
source port = af00 = 44800
destination port = bc06 = 48134
```

## 4. IEEE Floating Point

### 4.1. Deconstruction

f32
01000010001010100000000000000000
```
0 sign (1 bit) -> positive
10000100 exponent (8 bits) -> 132 - 127 (bias) = 5, 2^5 = 32
01010100000000000000000 fraction/mantissa (23 bits) = 1 + 0.25 + 0.0625 + 0.015625 = 0.328125

32 * 1.328125
42.5
```

## 5. Character encodings

### 5.1. Snowman

☃ = 3 (1 for length of bytes, 2 for the unicode character)

### 5.2. Hello again hellohex

first 5 bytes = `hello`
