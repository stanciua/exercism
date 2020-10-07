using System;
using System.Collections.Generic;

public static class VariableLengthQuantity
{
    public static uint[] Encode(uint[] numbers)
    {
        var encodeOne = new List<uint>();
        var encodeMany = new List<uint>();
        foreach (var n in numbers)
        {
            uint number = n;
            do
            {
                encodeOne.Insert(0, number & 0x7F);
                number >>= 7;
            } while (number != 0);

            if (encodeOne.Count > 1)
            {
                for (int i = 0; i < encodeOne.Count - 1; i++)
                {
                    encodeOne[i] |= 0x80;
                }
            }

            encodeMany.AddRange(encodeOne);
            encodeOne.Clear();
        }


        return encodeMany.ToArray();
    }

    public static uint[] Decode(uint[] bytes)
    {
        int validSequence = 0;
        uint decoded = 0;
        var decodeMany = new List<uint>();
        foreach (var t in bytes)
        {
            decoded |= (t & 0x7F);
            if ((t & 0x80) != 0)
            {
                validSequence++;
                decoded <<= 7;
                continue;
            }
            validSequence = 0;
            decodeMany.Add(decoded);
            decoded = 0;
        }

        if (validSequence != 0)
        {
            throw new InvalidOperationException("Invalid sequence of bytes received.");
        }
        
        return decodeMany.ToArray();
    }
}