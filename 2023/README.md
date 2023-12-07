# Thoughts

### Day 1
Part 2 was surprisingly difficult for a day one problem. So my solution is probably not the most efficient because I am 
trying to find a match for every single number and only then trying to find the earliest/latest one of the bunch.
I think trying to walk up/down the input and finding the earliest match right away might be more efficient but not by much.

### Day 2
Compared to Yesterday, today was a walk in the park. I wrote the first part as a two pass "parse then process" flow but it 
might be compiled into a single pass, in any case it is very easy to write it in a single pass.
The second part is written the same way and can also be done in an explicit single pass very easily.

### Day 3
This one was __hard__. Working on 2D grid is very weird. First part was not too hard using hashmap but the second part was
not easy. I struggled quite a bit to land on the data structure that would allow me to ergonomically get the numbers adjacent
to the gears. In the end I landed with A `HashMap<(x, y), Number>` where `Number` is actually a struct containing an id in addition
to the actual value. This id is used to remove duplicate of the same number when retrieving neighbors. For example, this :
```
.....
..123
..*..
.45..
```
this is stored like :
```
gears : (2, 2)
numbers : [
    (2, 1):(id: 1, value: 123)
    (3, 1):(id: 1, value: 123)
    (4, 1):(id: 1, value: 123)
    (1, 3):(id: 2, value: 45)
    (2, 3):(id: 2, value: 45)
```

This leads to some duplication in the data but makes processing it way easier. In this case I can just retreive all the neighbors 
of the gears and filter out the duplicate ids and that's it ! I believe it is pretty efficent in respect to time.

