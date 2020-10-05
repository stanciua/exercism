using System;
using System.Runtime.InteropServices;

public class Clock : IEquatable<Clock>
{
    private readonly int _hours;
    private readonly int _minutes;

    public Clock(int hours, int minutes)
    {
        // take into account wrapping around multiple times
        int h = minutes / 60;
        int m = minutes % 60;
        _hours = (hours + h) % 24;
        _minutes = m;
        // take into account negative hours and minutes
        if (_hours < 0)
        {
            _hours = 24 + _hours;
        }

        if (_minutes < 0)
        {
            if (_hours == 0)
            {
                _hours = 24;
            }
            _hours--;
            _minutes = 60 + _minutes;
        }
    }

    public Clock Add(int minutesToAdd)
    {
        int h = (_minutes + minutesToAdd) / 60;
        int m = (_minutes + minutesToAdd) % 60;
        h = (_hours + h) % 24;
        return new Clock(h, m);
    }

    public Clock Subtract(int minutesToSubtract)
    {
        int h = (_minutes - minutesToSubtract) / 60;
        int m = (_minutes - minutesToSubtract) % 60;
        h = (_hours - Math.Abs(h)) % 24;
        if (h < 0)
        {
            h = 24 + h;
        }

        if (m < 0)
        {
            if (h == 0)
            {
                h = 24;
            }

            h--;
            m = 60 + m;
        }

        return new Clock(h, m);
    }

    public bool Equals(Clock other)
    {
        if (ReferenceEquals(null, other))
        {
            return false;
        }

        if (ReferenceEquals(this, other))
        {
            return true;
        }

        return _hours == other._hours && _minutes == other._minutes;
    }

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(null, obj))
        {
            return false;
        }

        if (ReferenceEquals(this, obj))
        {
            return true;
        }

        if (obj.GetType() != this.GetType())
        {
            return false;
        }

        return Equals((Clock)obj);
    }

    public override int GetHashCode() => HashCode.Combine(_hours, _minutes);

    public override string ToString() => $"{_hours:D2}:{_minutes:D2}";
}