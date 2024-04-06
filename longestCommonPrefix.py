# Define a function to find the longest common prefix among a list of strings
def longest_common_prefix(strings):
    if not strings:
        return ""

    # Find the shortest string in the list
    shortest = min(strings, key=len)

    for i, char in enumerate(shortest):
        for string in strings:
            if string[i] != char:
                return shortest[:i]
    
    return shortest

def main():
    strings = input("Enter the strings separated by space: ").split()
    prefix = longest_common_prefix(strings)
    print("The longest common prefix is:", prefix)

if __name__ == "__main__":
    main()
