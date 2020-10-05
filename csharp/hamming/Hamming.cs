using System;

public static class Hamming
{
    public static int Distance(string firstStrand, string secondStrand)
    {
        if (firstStrand.Length != secondStrand.Length)
        {
            throw new ArgumentException("Length of the two strands is not identical.");
        }

        int distance = 0;
        for (int i = 0; i < firstStrand.Length; i++)
        {
            if (firstStrand[i] != secondStrand[i])
            {
                distance++;
            }
        }

        return distance;
    }
}