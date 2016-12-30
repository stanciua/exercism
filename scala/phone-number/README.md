# Phone Number

Write a program that cleans up user-entered phone numbers so that they can be sent SMS messages.

The rules are as follows:

- If the phone number is less than 10 digits assume that it is bad
  number
- If the phone number is 10 digits assume that it is good
- If the phone number is 11 digits and the first number is 1, trim the 1
  and use the last 10 digits
- If the phone number is 11 digits and the first number is not 1, then
  it is a bad number
- If the phone number is more than 11 digits assume that it is a bad
  number

We've provided tests, now make them pass.

Hint: Only make one test pass at a time. Disable the others, then flip
each on in turn after you get the current failing one to pass.

The Scala exercises assume an SBT project scheme. The exercise solution source
should be placed within the exercise directory/src/main/scala. The exercise
unit tests can be found within the exercise directory/src/test/scala.

To run the tests simply run the command `sbt test` in the exercise directory.

For more detailed info about the Scala track see the [help
page](http://exercism.io/languages/scala).


## Hints
For simplicity and readability: Consider using the Scala collection functions instead of Java's `String` methods. Remember that in Scala a `String` is implicitly also a `Seq[Char]`, so you can call them as easily as the `String` methods.

Some examples:
- `filter` instead of `replaceAll`
- `take`, `takeRight`, `drop`, `head`, `tail` instead of `substring`

Another idea worth exploring might be to change the `String` into a `List[Char]`
and then use [pattern matching](http://alvinalexander.com/scala/how-to-use-lists-nil-cons-scala-match-case-expressions) with the `::` operator.

## Source

Event Manager by JumpstartLab [http://tutorials.jumpstartlab.com/projects/eventmanager.html](http://tutorials.jumpstartlab.com/projects/eventmanager.html)

## Submitting Incomplete Problems
It's possible to submit an incomplete solution so you can see how others have completed the exercise.

