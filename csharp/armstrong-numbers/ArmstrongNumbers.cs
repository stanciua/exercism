using System;

public static class ArmstrongNumbers
{
    public static bool IsArmstrongNumber(int number)
    {
        int original = number;
        var length = number.ToString().Length;
        var calculated = 0;
        var divider = (int)Math.Pow(10, (length - 1));
        while (number != 0)
        {
            calculated += (int)Math.Pow(number / divider, length);
            number %= divider;
            divider /= 10;
        }

        return original == calculated;
    }
}