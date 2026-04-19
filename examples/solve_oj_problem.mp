fn fib(num) {
    if num <= 1 {
        return num;
    }
    fib(num - 1) + fib(num - 2)
}
let n = int(input())
let ans = fib(n)
print(ans)