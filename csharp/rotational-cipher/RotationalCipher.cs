using System;
using System.Collections.Generic;
using System.Text;

public static class RotationalCipher
{
    private const string LowerCaseLetters = "abcdefghijklmnopqrstuvwxyz";
    private const string UpperCaseLetters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    public static string Rotate(string text, int shiftKey)
    {
        var cipher = new StringBuilder();
        foreach (var c in text)
        {
            if (char.IsLower(c))
            {
                cipher.Append(LowerCaseLetters[(LowerCaseLetters.IndexOf(c) + shiftKey) % 26]);
            }
            else if (char.IsUpper(c))
            {
                cipher.Append(UpperCaseLetters[(UpperCaseLetters.IndexOf(c) + shiftKey) % 26]);
            }
            else
            {
                cipher.Append(c);
            }
        }

        return cipher.ToString();
    }
}