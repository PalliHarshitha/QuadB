# Define a function to check if the given string is palindrome or not
def is_palindrome(s):
    # Convert the string to lowercase and remove non-alphanumeric characters
    s = ''.join(char.lower() for char in s if char.isalnum())
    # Compare the string with its reverse
    return s == s[::-1]

def main():
    string = input("Enter a string to check if it's a palindrome: ")
    if is_palindrome(string):
        print("The string is a palindrome.")
    else:
        print("The string is not a palindrome.")

if __name__ == "__main__":
    main()
