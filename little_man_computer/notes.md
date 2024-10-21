# Little Man Computer

## 9 instructions

#### LDA
  $x -> A

#### STA
  A -> $x

#### ADD
  A + $x -> A

#### SUB
  A - $x -> A

#### INP
  I -> A

#### OUT
  A -> O

#### HLT
  Stop.

#### BRZ
  if A == 0:
    x -> PC

#### BRP
  if A >= 0:
    x -> PC

#### BRA
  x -> PC

#### DAT
  reserve mailbox


## For Loop
```LMC
BRA 2
DAT 1
INP
OUT
SUB 0
BRP 3
HLT

```
## Sum
```asm
00| BRA 4   ; skip
01| DAT 1   ; 1 for decrement
02| DAT 0   ; Total
03| DAT 0   ; index
04| INP
05| STA 3
06| ADD 2
07| STA 2
08| LDA 3 
09| SUB 1 
10| BRP 5 
11| LDA 2 
12| OUT
```

## Multiplication
```asm
00| BRA 4   ; skip
01| DAT 1   ; 1 for decrement
02| DAT 0   ; Total
03| DAT 0   ; index

05| INP
06| STA 3
07| ADD 2
08| STA 2
09| LDA 3 
10| SUB 1 
11| BRP 5 
12| LDA 2 
13| OUT
```





