import sys

ab = 0
_00 = 0

def do(first=False):
    global ab
    global _00
    if not first:
        print("invalid.")
    code = input("Enter code A[B or S]xxx, or x to stop inputting > ").upper()

    if code == "X":
        return

    if len(code) != 5:
        return do()

    if not(code[0:2] in ["AB", "AS"]):
        return do()

    if code[0:2] == "AB":
        ab += 1
        if code[3:5] == "00":
            _00 += 1

    return code

code = do(True)
while code != None:
    code = do(True)
print("AB: ", ab,"\n00:",_00)
