import math

def is_armstrong_number(number):

    if number == 0:
        return True # Since 0**1 is always 0.

    digit_count = math.ceil(math.log(number, 10))
    digits = [(number % (10**x)) // (10**(x - 1)) for x in range(1, digit_count + 1)]
    return sum([x**digit_count for x in digits]) == number