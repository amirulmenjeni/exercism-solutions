# Assume correct input (i.e., all sides are positive; list has size of 3 elements, etc.)

def is_valid_triangle(sides):
    return sum(sides) > 0 and \
           (sides[0] + sides[1] >= sides[2]) and \
           (sides[1] + sides[2] >= sides[0]) and \
           (sides[0] + sides[2] >= sides[1])

def equilateral(sides):
    return is_valid_triangle(sides) and all(side == sides[0] for side in sides)

def isosceles(sides):
    return is_valid_triangle(sides) and len(list(dict.fromkeys(sides))) < 3

def scalene(sides):
    return is_valid_triangle(sides) and len(list(dict.fromkeys(sides))) == 3
