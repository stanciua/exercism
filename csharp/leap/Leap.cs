using System;

public static class Leap
{
    public static bool IsLeapYear(int year)
    {
        bool isLeapYear = year % 4 == 0;

        if (year % 100 == 0)
        {
            isLeapYear = false;
        }

        if (year % 400 == 0)
        {
            isLeapYear = true;
        }

        return isLeapYear;
    }
}