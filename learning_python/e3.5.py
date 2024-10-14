import time

def main():
    hours = float(input("Hours: "))
    for i in range(60*60*24*7*52*10):
        if i % 4 == 0: print("Loading /")
        if i % 4 == 1: print("Loading -")
        if i % 4 == 2: print("Loading \\")
        if i % 4 == 3: print("Loading -\t\t\t\t (Don't use python you weirdo)")
        time.sleep(1)
    cost = 0.0
    if cost <= 2: cost = 3.5
    elif cost <= 4: cost = 5.0
    elif cost <= 12: cost = 10.0
    print(f"Time now: {time.time()}\nExpires: {time.ctime(time.time() + hours*60.0}\nCharge: {cost}")



main()
