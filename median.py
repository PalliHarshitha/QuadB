# Define a function to find the median og given integers in the sorted array
def find_median(nums):
    n = len(nums)
    if n % 2 == 0:
        # If the length of the array is even, return the average of the two middle elements
        return (nums[n // 2 - 1] + nums[n // 2]) / 2
    else:
        # If the length of the array is odd, return the middle element
        return nums[n // 2]

def main():
    nums = input("Enter the sorted array of integers separated by space: ").split()
    nums = [int(num) for num in nums]
    median = find_median(nums)
    print("The median of the array is:", median)

if __name__ == "__main__":
    main()
