# Define a function to check the first occurence of an integer in the given sorted array
def first_occurrence(nums, target):
    left, right = 0, len(nums) - 1
    result = -1
    
    while left <= right:
        mid = (left + right) // 2
        
        if nums[mid] == target:
            result = mid
            right = mid - 1  # Continue searching towards left
        elif nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return result

def main():
    nums = list(map(int, input("Enter the sorted array of integers separated by space: ").split()))
    target = int(input("Enter the number to find its first occurrence: "))
    
    index = first_occurrence(nums, target)
    if index != -1:
        print(f"The first occurrence of {target} is at index:", index)
    else:
        print(f"{target} is not found in the array.")

if __name__ == "__main__":
    main()
