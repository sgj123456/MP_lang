fn add(x, y) {
    return x + y
}

fn subtract(x, y) {
    return x - y
}

fn multiply(x, y) {
    return x * y
}

fn divide(x, y) {
    return x / y
}

print("add(10, 5):", add(10, 5))
print("subtract(10, 5):", subtract(10, 5))
print("multiply(10, 5):", multiply(10, 5))
print("divide(10, 5):", divide(10, 5))

fn greet(name) {
    return "Hello, " + name + "!"
}

print(greet("World"))
print(greet("Alice"))

fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print("factorial(5):", factorial(5))
