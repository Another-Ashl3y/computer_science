def main():
    purchase = float(input("Purcahse: "))
    discount = - (int(purchase >= 100.0 and purchase < 200.0)*purchase*0.05) - (int(purchase >= 200.0)*purchase*0.1)
    print(f"Value: {purchase}\nDiscount: {-discount/purchase}\nTotal: {purchase+discount}")


if __name__=="__main__":
    main()
