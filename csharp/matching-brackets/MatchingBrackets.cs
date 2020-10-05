using System;
using System.Collections.Generic;
using System.Runtime.InteropServices.ComTypes;
using Xunit.Sdk;

public static class MatchingBrackets
{
    public static bool IsPaired(string input)
    {
        List<char> stack = new List<char>();
        foreach (var c in input)
        {
            switch (c)
            {
                case '[':
                case '{':
                case '(':
                    stack.Add(c);
                    break;
                case ']':
                case '}':
                case ')':
                {
                    if ( stack.Count != 0 && MatchBracket(c) == stack[^1])
                    {
                        stack.RemoveAt(stack.Count - 1);
                    }
                    else
                    {
                        return false;
                    }

                    break;
                }
            }
        }

        return stack.Count == 0;
    }

    private static char MatchBracket(char c) => c switch
    {
        '}' => '{',
        ')' => '(',
        ']' => '[',
        _ => throw new ArgumentException("Invalid bracket received.")
    };
}