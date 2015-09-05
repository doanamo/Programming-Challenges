import random

print("Reverse guessing game!")
print("Use [Q]uit command to exit.\n");

quitting = False

tries = 0
floor = 1
ceiling = 100

while not quitting:
    guess = random.randint(floor, ceiling)
    tries += 1

    while True:
        print("Is your number {0:03}? ".format(guess), end='')
        print("[Y]es/[L]ower/[H]igher : ", end='');
        command = input()

        if command == 'y' or ceiling == floor:
            print("Computer guessed the number in {0} tries!".format(tries))
            quitting = True
            break
        elif command == 'l':
            ceiling = guess - 1
            break
        elif command == 'h':
            floor = guess + 1
            break
        elif command == 'q':
            quitting = True
            break
