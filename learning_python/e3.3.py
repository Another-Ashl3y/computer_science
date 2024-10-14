import random

def throw() -> (int, int):
    return (random.randint(1,6), random.randint(1,6))

def main():
    score = throw()
    print(score)
    if score[0] == score[1]:
        print(f"You threw a double!\nYour score is {sum(score)*2}")
        return
    print(f"Your score is {sum(score)}")


if __name__=="__main__":
    main()
