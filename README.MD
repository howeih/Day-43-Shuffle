Day 43: Shuffle
<br>
In machine learning we often need to shuffle data. For example, if we are about to make a train/test split and the data were sorted by category beforehand, we might end up training on just half of the classes. That would be bad.
<br>
Uniform shuffle guarantees that every item has the same chance to occur at any position. Seems like an easy task, but it requires a bit of thinking.
<br>
Hasty solution would be to cycle through all N positions, each time generate random value in range [0, N) and swap the current position with a random one. Is that uniform?
<br>
We select out of N^N shuffles, but there are only N! permutations. Obviously, we can’t get uniform distribution in this way and some items will have its preferences.
<br>
What we need, instead, is to get a random permutation. To find one, we cycle through, at position i generate a random value in range [i, N) and swap the two positions. Is that uniform?
<br>
p(0) = 1/N
<br>
p(1) = (N-1)/N * 1/(N-1) = 1/N
<br>
p(2) = (N-1/N) * (N-2)/(N-1) * 1/(N-2) = 1/N
<br>
Each item has the chance 1/N to get at position 0. Each item also has the chance (N-1)/N not to get there. Hence the chance for position 1 is the same, again, 1/N. This condition holds for all the array.