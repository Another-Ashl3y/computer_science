
def main():
    length = float(input("Length > "))
    width = float(input("Width > "))

    area = length*width

    if width == length:
        print(f"This is a square of area: {area}")
        return
    print(f"This is a rectangle of area: {area}")

if __name__=="__main__":
    main()

