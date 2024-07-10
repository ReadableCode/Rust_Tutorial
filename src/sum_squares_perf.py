def sum_of_squares(n):
    return sum(i * i for i in range(1, n + 1))


if __name__ == "__main__":
    result = sum_of_squares(10_000_000)
    print(result)
