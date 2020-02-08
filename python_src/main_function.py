import time

start_time = time.time()
number = 0
pi = 0
while number <= 100000000:
    pi = pi + ((-1) ** number * 1 / (2 * number + 1))
    number = number + 1
print('pi = ', pi * 4)
end_time = time.time()
run_time = end_time - start_time
print('{0}sec'.format(run_time))