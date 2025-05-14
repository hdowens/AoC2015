## Day 05

## My approach
Using python's in build `all` and `any` made it quite easy to string the rules together for each string.

## Regex Approach
Online someone posted solution using pure regex. I only know regex to pattern match n-len digits so I was interested to see it being used for bigger substring matching, particularly rule 2.

### Part 2:

In part 2 of this puzzle, there were two conditions for a nice string

1. Must have a repeating distinct two length sub string. The sub string must be distinct, that is to say they cannot share any letters. I.e. the string `aaa` does not pass this rule, `aa` and `aa` share a common `a`.
2. Must have a string in the form `xyx` where the first and last letter match, with exactly one letter between them.

So, this is what someone did online:
```
count = sum(
  1 for s in datalines
  if len(re.findall(r"([a-z]{2}).*\1", s))
  and re.findall(r"([a-z]).\1", s)
)

```

#### Rule 1:
The regex patterns that matches rule 1 is : `([a-z]{2}).*\1`. 

`([a-z]{2})`.The first thing is `[a-z]`. This matches for any lower-case latin alphabet character. All strings in this problem are known to be lowercase. The `{2}` matches for the previous token exactly twice. So, we are looking for any letter substring at this point. N.B., this is a "capturing group", as its w rapped in brackets. 

`.*` matches for any number of characters, and is a greedy approach. Tied together with `\1`, it back-references to the first capturing group `([a-z]{2})`. So, this just looks for sub strings with a repeating 2 letter group!

Note, the dev writes `len(re.findall(<pat>))` which, if any return, will pass the test. Nice.

#### Rule 2:
The regex pattern that matches rule 2 is : `([a-z]).\1` 

`([a-z])` without the `{2}`, means that it matches to exactly one letter. We then have `.` which matches any single character, note without the `*` it only looks for one, instead of greedily any. Then `\1` back-references the first group. 


Note these arent very strict, the problem assumed kinda that the strings are all alphabetic and the rule 2 pattern would also match for `c_C`, which probably isnt a nice string!

## CPP Approach
Decided to dust off the cpp knowledge. There are a lot of new convenient algorithms implemented that make this nice. The lambdas also help. Big key here is using string views, make it much more efficient.