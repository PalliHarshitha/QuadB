# Define a function to find the kth smallest element in the array
def kth_smallest(nums, k):
    nums.sort() # Sort the array in ascending order
    return nums[k - 1]  # Return the kth smallest element (0-indexed)

# Main function to take user input and call the kth_smallest function
def main():
    nums = input("Enter the array elements separated by space: ").split()
    nums = [int(num) for num in nums]
    k = int(input("Enter the value of k: "))
    
    # Call the kth_smallest function with the input array and k
    kth = kth_smallest(nums, k)
    print(f"The {k}th smallest element in the array is: {kth}")

# Entry point of the program
if __name__ == "__main__":
    main()
