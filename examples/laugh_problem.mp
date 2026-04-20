/*
Description
Xiao Ha's laugh consists of alternating a and h with no consecutive duplicates. Valid examples: ahahah, aha, a, h. Invalid: aa, hh, strings with other letters. Given a string S of length N, find the length of the longest valid laugh substring.
Input
First line: integer N (1 ≤ N ≤ 1e5)
Second line: string S (lowercase letters only)
Output
Print the maximum length of a valid laugh.
*/
let n_input = input()
let n = int(n_input)
let s = input()

let max_len = 0
let current_len = 0
let prev_char = ""

let i = 0
while i < n {
    let ch = s[i]

    if (ch == "a") || (ch == "h") {
        if current_len == 0 {
            current_len = 1
            prev_char = ch
        } else {
            if ch != prev_char {
                current_len = current_len + 1
                prev_char = ch
            } else {
                current_len = 1
                prev_char = ch
            }
        }

        if current_len > max_len {
            max_len = current_len
        }
    } else {
        current_len = 0
        prev_char = ""
    }

    i = i + 1
}

print(max_len)
