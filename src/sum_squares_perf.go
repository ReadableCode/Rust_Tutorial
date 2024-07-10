package main

import (
    "fmt"
    "math/big"
)

// sumOfSquares calculates the sum of squares from 1 to n using big.Int.
func sumOfSquares(n int) *big.Int {
    sum := big.NewInt(0)
    for i := 1; i <= n; i++ {
        square := big.NewInt(int64(i))
        square.Mul(square, square)
        sum.Add(sum, square)
    }
    return sum
}

func main() {
    result := sumOfSquares(10000000)

    fmt.Println(result)
}