let a = 10
let b = 20

if a > b {
    print("a is greater than b")
} else {
    print("a is not greater than b")
}

if a < b {
    print("a is less than b")
}

let score = 85
if score >= 90 {
    print("Grade: A")
} else if score >= 80 {
    print("Grade: B")
} else if score >= 70 {
    print("Grade: C")
} else {
    print("Grade: F")
}

let is_valid = true
if is_valid {
    print("Valid!")
}

if false {
    print("This will not print")
} else {
    print("This will print")
}

let result = if 5 > 3 { "yes" } else { "no" }
print("Inline if result:", result)
