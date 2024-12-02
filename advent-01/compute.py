list1 = []
list2 = []

with open("input") as f:
    for line in f.readlines():
        a, b = line.split("   ")
        list1.append(int(a))
        list2.append(int(b))

list1.sort()
list2.sort()

sum = 0

for x, y in zip(list1, list2):
    sum += abs(x-y)

print(sum)