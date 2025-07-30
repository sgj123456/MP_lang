let a = 10
let b = 20
fn add(x, y) {
    x = x + 1
    return x + y
    a = 100 // you are able to modify 'a' here, but it won't affect the global variable 'a'.
    x + y + a
}
let c = add(a, b)
print(c)
// return "Hello, world!" 'return' will instandly exit the function and return the value.
let d = [1, 2, 3, 4, 5]
print(d)
let f = { "name": "John", "age": 30 }
let e = while (a < 20) {
    a = a + 1
    print(a)
    a
};
print(e)
if 1 > 2 {3} else {4;}