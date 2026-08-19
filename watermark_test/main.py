# REQUIREMENTS: Python 3.6+ (no external dependencies)
#
# DESCRIPTION:
#   Hello World project that greets the user, generates a random nickname
#   from word arrays, and offers a small menu of extra random novelties:
#   a lucky number, a random fun fact, and a random fortune message.
#
# USAGE:
#   python main.py
#
# EXAMPLE USAGE:
#   python main.py
#   > Enter your name: Mateusz
#   > Hello, Mateusz! Your new nickname is: Sneaky Turbo Badger
#   > What would you like to do next? [1] New nickname [2] Lucky number [3] Fun fact [4] Fortune [5] Quit
#
# --- SCRIPT ---

import random

# Word arrays used to assemble a random nickname
ADJECTIVES = [
    "Sneaky", "Turbo", "Fuzzy", "Mighty", "Silent",
    "Wobbly", "Grumpy", "Sparkly", "Feral", "Cosmic",
]

NOUNS = [
    "Badger", "Falcon", "Noodle", "Wizard", "Potato",
    "Ninja", "Goblin", "Panda", "Rocket", "Pickle",
]

TITLES = [
    "the Bold", "the Wise", "the Swift", "the Brave", "the Weird",
    "of Doom", "of Chaos", "the Great", "the Unstoppable", "the Curious",
]

# Greeting templates used to vary how the user is welcomed
GREETINGS = [
    "Hello, {name}!",
    "Greetings, {name}!",
    "Well met, {name}!",
    "Yo {name}, what's up?",
    "Ah, {name}! Good to see you.",
]

# Random fun facts shown when the user picks that menu option
FUN_FACTS = [
    "Honey never spoils, archaeologists have found 3000 year old honey that's still edible.",
    "A group of flamingos is called a 'flamboyance'.",
    "Octopuses have three hearts and blue blood.",
    "Bananas are berries, but strawberries aren't.",
    "The Eiffel Tower can grow taller in summer due to thermal expansion.",
    "Wombat poop is cube shaped.",
    "A day on Venus is longer than a year on Venus.",
]

# Random fortune style messages shown when the user picks that menu option
FORTUNES = [
    "A great opportunity is hiding in your next bug fix.",
    "Beware of off by one errors today.",
    "Your code will compile on the first try. Eventually.",
    "A stranger will bring you a pull request full of surprises.",
    "Patience with your compiler will be rewarded.",
    "Today is a good day to write a comment you'll thank yourself for later.",
]


def generate_nickname():
    # Pick one random word from each array and combine them into a nickname
    adjective = random.choice(ADJECTIVES)
    noun = random.choice(NOUNS)
    title = random.choice(TITLES)
    return f"{adjective} {noun} {title}"


def generate_greeting(user_name):
    # Pick a random greeting template and fill in the user's name
    template = random.choice(GREETINGS)
    return template.format(name=user_name)


def generate_lucky_number():
    # Produce a random "lucky" number in a friendly range
    return random.randint(1, 100)


def get_random_fun_fact():
    # Pick and return one random fun fact from the list
    return random.choice(FUN_FACTS)


def get_random_fortune():
    # Pick and return one random fortune message from the list
    return random.choice(FORTUNES)


def print_menu():
    # Display the available follow-up actions to the user
    print("\nWhat would you like to do next?")
    print("  [1] Generate a new nickname")
    print("  [2] Get a lucky number")
    print("  [3] Hear a random fun fact")
    print("  [4] Read your fortune")
    print("  [5] Quit")


def main():
    # Prompt the user for their name, defaulting to "Stranger" if left blank
    user_name = input("Enter your name: ").strip()
    if not user_name:
        user_name = "Stranger"

    greeting = generate_greeting(user_name)
    nickname = generate_nickname()
    print(f"{greeting} Your new nickname is: {nickname}")

    # Loop through a simple menu until the user chooses to quit
    while True:
        print_menu()
        choice = input("Choose an option (1-5): ").strip()

        if choice == "1":
            print(f"Your new nickname is: {generate_nickname()}")
        elif choice == "2":
            print(f"Your lucky number is: {generate_lucky_number()}")
        elif choice == "3":
            print(f"Fun fact: {get_random_fun_fact()}")
        elif choice == "4":
            print(f"Fortune: {get_random_fortune()}")
        elif choice == "5":
            print(f"Goodbye, {user_name}!")
            break
        else:
            print("Invalid choice, please pick a number from 1 to 5.")


if __name__ == "__main__":
    main()
