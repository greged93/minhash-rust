# Introduction

The concept of min hash is used in order to evaluate the similarity between sets of data. This can be done by evaluating the
Jaccard similarity coeffecient between each set. Given set A and a set B, the coefficient can be evaluated by $J(A, B)=\frac{A\bigcap B}{A\cup B}$.

The goal of MinHash is to evaluate the similarity coefficient without needing to directly compare sets A and B. This can be done by the use of
a hash function. By taking the hash of all the elements and assuming no hash collision, we can then take the minimum of the hashing values for
A and B. If this minimum value is equal, in other words $h_{min}(A) = h_{min}(B)$, then the value which gives this minimum value is part of
$A\bigcap B$. The probability of this happening is exactly the value of the Jaccard similarity coefficient: $J(A, B)=Pr[h_{min}(A) = h_{min}(B)]$.

# Algorithm

In the current implementation of the code, a mapping is first applied to the data in order to obtain distinct values. The following mappings are used:

-   Instructions: $(i + 1)*256 + C$ where $C$ is the amount of time instruction $i$ as been encountered in the instruction array.
-   Mechs: $x * 256 + y*32 + C + offset_{mech}$ where $x$ and $y$ are the position of the mech on the board, $C$ is the amount of time a mech at location $(x,y)$
    as been encountered and $offset_{mech}$ is currently choosen as 2048.
-   Operators: $t * 256 + x * 16 + y + offset_{operator}$ where $t$ is the type of the operator, $x$ and $y$ are the position of the operator on the board and
    $offset_{operator}$ is currently choosen as 4096.

Once this mapping as been applied for both solutions being compared, the MinHash value of the solutions can be calculated from the mappings. The
threshold for solutions that are too similar is taken as 0.9 for the Jaccard similarity coefficient.
