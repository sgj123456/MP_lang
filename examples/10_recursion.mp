fn fib(n) {
    if n <= 1 {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}
print("Fibonacci sequence:")
let i = 0;
while i < 10 {
    print("fib(", i, "):", fib(i))
    i = i + 1
}
fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
print("\nFactorials:")
print("factorial(0):", factorial(0))
print("factorial(1):", factorial(1))
print("factorial(5):", factorial(5))
print("factorial(10):", factorial(10))
fn power(base, exp) {
    if exp == 0 {
        return 1
    }
    return base * power(base, exp - 1)
}
print("\nPowers:")
print("power(2, 10):", power(2, 10))
print("power(3, 4):", power(3, 4))
