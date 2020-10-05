using System;
using System.Collections.Generic;
using System.Security.Claims;

public class SpiralMatrix
{
    public static int[,] GetMatrix(int size)
    {
        int number = 1;
        int[,] spiral = new int[size, size];
        int i = 0;
        int j = 0;
        int level = 0;
        do
        {
            // go right
            for (j = level; j < size - level; j++)
            {
                spiral[i, j] = number++;
            }
            j--;
            // go down
            for (i = level + 1; i < size - level; i++)
            {
                spiral[i, j] = number++;
            }

            i--;
            // go left
            for (j--; j >= level; j--)
            {
                spiral[i, j] = number++;
            }

            j++;
            // go up
            for (i--; i > level; i--)
            {
                spiral[i, j] = number++;
            }
            i++;
            level++;
        } while (number - 1 != size * size);

        return spiral;
    }
}