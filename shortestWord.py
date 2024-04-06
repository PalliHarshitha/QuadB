# Define a function if the shortest word of all other words in the sentence
def shortest_word(sentence):
    # Split the sentence into words
    words = sentence.split()
    
    # If there are no words, return None
    if not words:
        return None
    
    # Find the shortest word using min() function with key=len
    shortest = min(words, key=len)
    
    return shortest

def main():
    sentence = input("Enter a string of words: ")
    shortest = shortest_word(sentence)
    
    if shortest is not None:
        print("The shortest word in the string is:", shortest)
    else:
        print("No words found in the string.")

if __name__ == "__main__":
    main()
