using System;
using System.Collections.Generic;

public static class NucleotideCount
{
    public static IDictionary<char, int> Count(string sequence)
    {
        var dictionary = new Dictionary<char, int>() {['A'] = 0, ['C'] = 0, ['G'] = 0, ['T'] = 0};
        foreach (var type in sequence)
        {
            if (!dictionary.ContainsKey(type))
            {
                throw new ArgumentException("Invalid DNA type received.");
            }
            dictionary[type]++;
        }

        return dictionary;
    }
}