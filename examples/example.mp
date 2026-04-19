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

let arr = [10, 20, 30, 40, 50]
print("Array:")
print(arr)
print("First element:")
print(arr[0])
print("Third element:")
print(arr[2])

let person = {"name": "Alice", "age": 25, "city": "Beijing"}
print("\nPerson object:")
print(person)
print("Name:")
print(person:name)
print("Age:")
print(person:age)

print("\nLength tests:")
print("Length of string 'hello':")
print(len("hello"))
print("Length of array:")
print(len(arr))

print("\nType tests:")
print("Type of 42:")
print(type(42))
print("Type of 3.14:")
print(type(3.14))
print("Type of 'hello':")
print(type("hello"))
print("Type of true:")
print(type(true))
print("Type of array:")
print(type(arr))
print("Type of nil:")
print(type(nil))

print("\nString conversion:")
print("Number 123 as string:")
print(str(123))
print("Float 3.14 as string:")
print(str(3.14))

let matrix = [[1, 2], [3, 4], [5, 6]]
print("\nMatrix:")
print(matrix)
print("matrix[1][0]:")
print(matrix[1][0])

let u1 = {"name": "Bob", "age": 30}
let u2 = {"name": "Carol", "age": 28}
let users = [u1, u2]
print("\nFirst user name:")
print(users[0]:name)
print("Second user age:")
print(users[1]:age)

print("\nAll features work correctly!")
