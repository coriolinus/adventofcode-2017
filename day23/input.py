b = 109900
c = 126900
h = 0

for b in range(b, c+17, 17):
   f = True

   for d in range(2, b):
       if b % d == 0:
           f = False

   if f == False:
      h += 1

   if b == c:
      break

print(h)
