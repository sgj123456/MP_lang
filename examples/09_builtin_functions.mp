print("=== Builtin Functions ===")

print("\n--- print ---")
print("Hello, World!")
print(42)
print(3.14)
print(true)
print([1, 2, 3])

print("\n--- input ---")
print("Input is disabled in demo")

print("\n--- int ---")
print("int(3.7):", int(3.7))
print("int(5.9):", int(5.9))
print("int(\"42\"):", int("42"))

print("\n--- float ---")
print("float(5):", float(5))
print("float(\"3.14\"):", float("3.14"))

print("\n--- str ---")
print("str(123):", str(123))
print("str(3.14):", str(3.14))
print("str(true):", str(true))
print("str([1,2,3]):", str([1, 2, 3]))

print("\n--- len ---")
print("len(\"hello\"):", len("hello"))
print("len([1,2,3,4]):", len([1, 2, 3, 4]))
print("len({\"a\":1, \"b\":2}):", len({"a": 1, "b": 2}))

print("\n--- type ---")
print("type(42):", type(42))
print("type(3.14):", type(3.14))
print("type(\"hello\"):", type("hello"))
print("type(true):", type(true))
print("type([1,2]):", type([1, 2]))
print("type({\"x\":1}):", type({"x": 1}))
print("type(nil):", type(nil))

print("\n--- random ---")
print("random():", random())
print("random(10):", random(10))
print("random(1, 100):", random(1, 100))

print("\n--- time ---")
print("time():", time())

print("\n--- push and pop ---")
let arr = [1, 2, 3]
print("Original array:", arr)
push(arr, 4)
print("After push(4):", arr)
let popped = pop(arr)
print("Popped value:", popped)
print("After pop:", arr)
