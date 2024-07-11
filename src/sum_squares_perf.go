package main

import (
    "encoding/json"
    "fmt"
    "math/big"
    "time"
)

type Output struct {
    Result         *big.Int `json:"result"`
    ElapsedTimeMs  float64  `json:"elapsed_time_ms"`
}

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
    startTime := time.Now()
    result := sumOfSquares(10000000)
    elapsedTime := time.Since(startTime).Seconds() * 1000 // in milliseconds

    output := Output{
        Result:        result,
        ElapsedTimeMs: elapsedTime,
    }

    jsonOutput, err := json.Marshal(output)
    if err != nil {
        fmt.Println("Error marshalling to JSON:", err)
        return
    }

    fmt.Println(string(jsonOutput))
}
