let i = 0
while i < 10 {
    i = i + 1
    if i == 3 {
        continue
    }
    if i == 7 {
        break
    }
    print("i =", i)
}

print("---")

let j = 0
while j < 5 {
    j = j + 1
    if j == 2 {
        break
    }
    print("j =", j)
}
print("Loop ended with j =", j)

let sum = 0
let k = 1
while k <= 10 {
    if k % 2 == 0 {
        k = k + 1
        continue
    }
    sum = sum + k
    k = k + 1
}
print("Sum of odd numbers 1-10:", sum)
