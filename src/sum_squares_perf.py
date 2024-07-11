import json
import time


def sum_of_squares(n):
    return sum(i * i for i in range(1, n + 1))


if __name__ == "__main__":
    start_time = time.time()
    result = sum_of_squares(10_000_000)
    end_time = time.time()
    elapsed_time_ms = (end_time - start_time) * 1000

    # Output the result and the time as JSON
    output = {"result": result, "elapsed_time_ms": elapsed_time_ms}
    print(json.dumps(output))
