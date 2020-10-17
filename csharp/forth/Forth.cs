using System;
using System.Collections.Generic;
using System.Linq;

public static class Forth
{
    private static readonly Stack<short> Stack = new Stack<short>();

    private static readonly Dictionary<string, List<string>> Words =
        new Dictionary<string, List<string>>();

    public static string Evaluate(IEnumerable<string> instructions)
    {
        Stack.Clear();
        Words.Clear();
        foreach (var instruction in instructions)
        {
            var tokens = instruction.ToUpper().Split();
            if (tokens[0] == ":" && tokens[^1] == ";")
            {
                ParseUserDefinedWords(tokens);
                continue;
            }

            var expandTokens = TranslateUserDefinedWords(tokens);

            foreach (var token in expandTokens)
            {
                if (short.TryParse(token, out var number))
                {
                    Stack.Push(number);
                    continue;
                }

                short op1, op2;
                switch (token)
                {
                    case "+":
                        if (Stack.Count < 2)
                        {
                            throw new InvalidOperationException(
                                "+ operation requires 2 arguments");
                        }

                        op1 = Stack.Pop();
                        op2 = Stack.Pop();
                        Stack.Push((short)(op2 + op1));
                        break;
                    case "-":
                        if (Stack.Count < 2)
                        {
                            throw new InvalidOperationException(
                                "- operation requires 2 arguments");
                        }

                        op1 = Stack.Pop();
                        op2 = Stack.Pop();
                        Stack.Push((short)(op2 - op1));
                        break;
                    case "*":
                        if (Stack.Count < 2)
                        {
                            throw new InvalidOperationException(
                                "* operation requires 2 arguments");
                        }

                        op1 = Stack.Pop();
                        op2 = Stack.Pop();
                        Stack.Push((short)(op2 * op1));
                        break;
                    case "/":
                        if (Stack.Count < 2)
                        {
                            throw new InvalidOperationException(
                                "/ operation requires 2 arguments");
                        }

                        op1 = Stack.Pop();
                        op2 = Stack.Pop();
                        if (op1 == 0)
                        {
                            throw new InvalidOperationException("Division by zero");
                        }

                        Stack.Push((short)(op2 / op1));
                        break;
                    case "DUP":
                        Dup();
                        break;
                    case "DROP":
                        Drop();
                        break;
                    case "SWAP":
                        Swap();
                        break;
                    case "OVER":
                        Over();
                        break;
                    default:
                        throw new InvalidOperationException("Invalid operation received");
                }
            }
        }

        return string.Join(' ', Stack.Reverse());
    }

    private static void Dup()
    {
        if (Stack.Count < 1)
        {
            throw new InvalidOperationException("DUP requires at least one element");
        }

        Stack.Push(Stack.Peek());
    }

    private static void Drop()
    {
        if (Stack.Count < 1)
        {
            throw new InvalidOperationException("DROP requires at least 1 element");
        }

        Stack.Pop();
    }

    private static void Swap()
    {
        if (Stack.Count < 2)
        {
            throw new InvalidOperationException("SWAP requires at least 2 elements");
        }

        var op1 = Stack.Pop();
        var op2 = Stack.Pop();
        Stack.Push(op1);
        Stack.Push(op2);
    }

    private static void Over()
    {
        if (Stack.Count < 2)
        {
            throw new InvalidOperationException("OVER requires at least 2 elements");
        }

        var op1 = Stack.Pop();
        var op2 = Stack.Peek();
        Stack.Push(op1);
        Stack.Push(op2);
    }

    private static void ParseUserDefinedWords(IReadOnlyList<string> tokens)
    {
        var isNumber = short.TryParse(tokens[1], out _);
        if (isNumber)
        {
            throw new InvalidOperationException(
                "User defined words cannot redefine numbers");
        }

        if (tokens.Count < 4)
        {
            throw new InvalidOperationException(
                "User defined word should have at least one operand");
        }

        var translation = new List<string>();
        for (var i = 2; i <= tokens.Count - 2; i++)
        {
            // if the token refers to an existing word, then replace it the content of the word
            if (Words.ContainsKey(tokens[i]))
            {
                translation.AddRange(Words[tokens[i]]);
            }
            else
            {
                translation.Add(tokens[i]);
            }
        }

        Words[tokens[1]] = translation;
    }

    private static IEnumerable<string> TranslateUserDefinedWords(IEnumerable<string> tokens)
    {
        var expandTokens = new List<string>();
        foreach (var token in tokens)
        {
            if (Words.ContainsKey(token))
            {
                expandTokens.AddRange(Words[token]);
                continue;
            }

            expandTokens.Add(token);
        }

        return expandTokens;
    }
}