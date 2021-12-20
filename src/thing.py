def readFile():
    inputArray = []
    with open("sonar.txt") as file:
        for line in file:
            inputArray.append(line.strip())

    return inputArray

####### pseudo code ########

#iterate through array
#if array[i] > array[i+1], increase counter by 1.
#return counter.

#time complexity: O(n)

def sonarSweep(inputArray):
    increaseCounter = 0
    for i in range(1, (len(inputArray))):
        if inputArray[i-1] < inputArray[i]:
            increaseCounter += 1
    return increaseCounter

if __name__ == "__main__":
    sonarSweep(readFile())