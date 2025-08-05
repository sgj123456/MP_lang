while true {
    answer = random(1, 100)
    print("Guess a number between 1 and 100: ")
    while true {
        guess = int(input())
        if guess == answer {
            print("Congratulations! You guessed the number!")
            break
        } else if guess < answer {
            print("Too low. Try again.")
        } else {
            print("Too high. Try again.")
        }
    }
}
