# wordle_solver
POC solver for https://www.powerlanguage.co.uk/wordle/

# Installation
Find a dictionary, cut all non five letter words out and name it `wordle_list.txt`.

# Usage
```
===== Key =====
GGGGG => Complete
__GR_ => If a letter is marked blank but already correct e.g. 'green'.
__G__ => Letter 3 is correct.
__Y__ => Letter 3 is a valid letter but in the wrong place.
_____ => All letters incorrect.
ERROR => Word suggestion invalid.
```
