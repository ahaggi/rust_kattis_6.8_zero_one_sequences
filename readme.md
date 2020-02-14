



# 0-1 Sequences



You are given a sequence, in the form of a string with characters 0, 1, and ? only. Suppose there are k ?s. Then there are 2^ k ways to replace each ? by a 0 or a 1, giving 2^ k different 0-1 sequences (0-1 sequences are sequences with only zeroes and ones). 

For each 0-1 sequence, define its number of inversions as the minimum number of adjacent swaps required to sort the sequence in non-decreasing order. In this problem, the sequence is sorted in non-decreasing order precisely when all the zeroes occur before all the ones. For example, the sequence 11010 has 5 inversions. We can sort it by the following moves: 11010 ->  11001 ->  10101 ->  01101 ->  01011 ->  00111. 

Find the sum of the number of inversions of the 2^ k sequences, modulo 1, 000, 000, 007 (10^9 + 7). ## Input The first and only line of input contains the input string, consisting of characters 0, 1, and ? only, and the input string has between 1 to 500, 000 characters, inclusive. ## Output Output an integer indicating the aforementioned number of inversions modulo 1,000,000,007.


## Input

The first and only line of input contains the input string, consisting of characters ‘0’, ‘1’, and ‘?’ only, and the input string has between 1 and 500 000 characters, inclusive.

## Output

Output an integer indicating the aforementioned number of inversions modulo 1,000,000,007.


<table class="sample" summary="sample data">

<tbody>

<tr>

<th>Sample Input 1</th>

<th>Sample Output 1</th>

</tr>

<tr>

<td>

<pre>?0?
</pre>

</td>

<td>

<pre>3
</pre>

</td>

</tr>

</tbody>

</table>



