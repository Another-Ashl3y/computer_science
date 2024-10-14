def main():
    options = ["Music", "History", "Design And Technology", "Exit"]

    print("Menu")

    for index, item in enumerate(options):
        print(f"{index+1}.\t{item}")

    selection = int(input("Please enter your choice: "))

    if not(selection - 1 in range(0, len(options))):
        print("Please select a valid option")
        main()
        return

    if selection == len(options):
        print("Goodbye")
        return

    print(f"You chose {options[selection-1]}")


if __name__=="__main__":
    main()
