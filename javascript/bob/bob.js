//
// This is only a SKELETON file for the "Bob" exercise. It's been provided as a
// convenience to get you started writing code faster.
//

// Bob is a lackadaisical teenager. In conversation, his responses are very limited.

// Bob answers 'Sure.' if you ask him a question.

// He answers 'Whoa, chill out!' if you yell at him.

// He says 'Fine. Be that way!' if you address him without actually saying anything.

// He answers 'Whatever.' to anything else.

var Bob = function() {};

Bob.prototype.hey = function(input) {
  if (input.search(/[a-zA-z]/) !== -1 && input.toUpperCase() === input) return "Whoa, chill out!"
  if (input.endsWith("?")) return "Sure."
  if (input.trim().length == 0) return "Fine. Be that way!"
  return "Whatever."
};

module.exports = Bob;
