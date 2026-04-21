struct Student { name, score }
let students = [{ "name": "Lihua", "score": 100 }, Student("Bob", 82), Student("Charlie", 78), Student("Diana", 91), Student("Eve", 88)];
fn print_student(student) {
    print("  Name:", student:name, " Score:", student:score)
}
print("=== All Students ===")
let i = 0;
while i < len(students) {
    print_student(students[i])
    i = i + 1
}
fn average_score(students) {
    let sum = 0;
    let j = 0;
    while j < len(students) {
        sum = sum + students[j]:score
        j = j + 1
    }
    return sum / len(students)
}
print("\nAverage score:", average_score(students))
fn find_best(students) {
    let best = students[0];
    let k = 1;
    while k < len(students) {
        if students[k]:score > best:score {
            best = students[k]
        }
        k = k + 1
    }
    return best
}
let best_student = find_best(students);
print("Best student:", best_student:name, " with score ", best_student:score)
fn filter_above(students, threshold) {
    let result = [];
    let l = 0;
    while l < len(students) {
        if students[l]:score >= threshold {
            push(result, students[l])
        }
        l = l + 1
    }
    return result
}
let excellent = filter_above(students, 90);
print("\nStudents with score >= 90:")
let m = 0;
while m < len(excellent) {
    print("  ", excellent[m]:name, ":", excellent[m]:score)
    m = m + 1
}
