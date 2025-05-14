import hashlib

def main():
    test = 'bgvyzdsv'
    for i in range(0, 5_000_000):
        if hashlib.md5(f"{test}{str(i)}".encode('utf-8')).hexdigest()[:6] == '000000':
            print("winnar: ", i)

if __name__ == "__main__":
    main()


 