def steps(number):
    if number < 0 or number == 0:
        raise ValueError("Only positive integers are allowed")

    n = number
    step = 0
    while n > 1:
        if n % 2 == 0:
            n = n // 2
        else:
            n = 3*n + 1
        step += 1
    return step
