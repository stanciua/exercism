using System;
using System.Collections.Generic;
using System.Linq;

public static class SumOfMultiples
{
    public static int Sum(IEnumerable<int> multiples, int max)
    {
        var numbers = new HashSet<int>();
        for (int n = 1; n < max; n++)
        {
            foreach (var m in multiples)
            {
                if (m == 0)
                {
                    continue;
                }
                if (n % m == 0)
                {
                    if (!numbers.Contains(n))
                    {
                        numbers.Add(n);
                    }
                } 
            }
        }

        return numbers.Sum();
    }
}