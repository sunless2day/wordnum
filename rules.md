Here is a problem-solving challenge:

# English-to-integer program

Create a program that allows the user to enter a number in plain English text (for example : "one hundred") and outputs the number as an integer.

## Examples :

- Example 1 :

```
Input :
>>> One hundred and one 
```

```
Output :
>>> 101 
```

- Example 2 :

```
Input :
>>> Three thousand one hundred and fifty-two
```

```
Output :
>>> 3152
```

- Example 3 :

```
Input :
>>> Nine hundred billion, thirty thousand, sixty-two
```

```
Output :
>>> 900,000,030,062
```

Note : Notice that in each different example, the program ignores commas (,)
Though the word "and" is optional

# RULES :

- You can write the program in any programming language
- The program must be able to translate any number in plain english from 0 to at least 999 Billion (You can optionally go higher)
- The code must be dynamic (as in algorithmic & flexible), not static (as in hardcoded & fixed)
- The code must ignore commas ','
- The code must only translate integers, not decimals. If the user inputs something like "One hundred point two", you can either make the program ignore the "point two" and simply interpret it as "One hundred", or you can re-prompt the user
- The challenge ends on 21 November 2026
