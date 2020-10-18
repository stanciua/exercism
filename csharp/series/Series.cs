using System;
using System.Collections.Generic;

public static class Series
{
    public static string[] Slices(string numbers, int sliceLength)
    {
        if (sliceLength > numbers.Length)
        {
            throw new ArgumentException("Slice length is too large");
        }
        if (sliceLength == 0)
        {
            throw new ArgumentException("Slice length cannot be zero");
        }
        if (sliceLength < 0)
        {
            throw new ArgumentException("Slice length cannot negative");
        }
        if (numbers.Length == 0)
        {
            throw new ArgumentException("Empty series are invalid");
        }

        var series = new List<string>();
        for (int i = 0; i < numbers.Length; i++)
        {
            if (i + sliceLength <= numbers.Length)
            {
                series.Add(numbers.Substring(i, sliceLength));
            }
        }

        return series.ToArray();
    }
}