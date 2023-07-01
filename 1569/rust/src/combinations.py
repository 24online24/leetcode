import functools

def multiply(n, m):
    product = 1
    for i in range(n, m+1):
        product *= i
    return product

def combinations(a, b):
    if a>b:
        multiply(a+1, a+b) / multiply(1, b)
    else:
        multiply(b+1,)

else:
