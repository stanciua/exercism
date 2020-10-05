using System;
using System.ComponentModel;
using System.Linq;

public static class Bob
{
    public static string Response(string statement)
    {
        string[] lines = statement.Split(new[] {Environment.NewLine}, StringSplitOptions.None);

        // work only with the last line, ignoring the rest of the lines
        string line = lines[^1].Trim();

        if (isBeeignYelledAt(line) && line.EndsWith('?'))
        {
            return "Calm down, I know what I'm doing!";
        }
        else if (isBeeignYelledAt(line))
        {
            return "Whoa, chill out!";
        }
        else if (line.EndsWith('?'))
        {
            return "Sure.";
        }
        else if (line.Length == 0)
        {
            return "Fine. Be that way!";
        }
        else
        {
            return "Whatever.";
        }
    }

    private static bool isBeeignYelledAt(string line)
    {
        string upperLine = line.ToUpper();
        return line.Any(char.IsLetter) && line.Trim().Length != 0 && upperLine == line;
    }
}