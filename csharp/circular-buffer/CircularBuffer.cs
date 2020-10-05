using System;

public class CircularBuffer<T>
{
    private readonly T[] _circularBuffer;
    private readonly int _capacity;
    private int _size;
    private int _start;
    private int _end;

    public CircularBuffer(int capacity)
    {
        _circularBuffer = new T[capacity];
        _capacity = capacity;
    }

    public T Read()
    {
        if (_size == 0)
        {
            throw new InvalidOperationException("Circular buffer is empty.");
        }

        _size--;
        T val = _circularBuffer[_start];
        _start = (_start + 1) % _capacity;
        return val;
    }

    public void Write(T value)
    {
        if (_size == _capacity)
        {
            throw new InvalidOperationException("The circular buffer is full.");
        }

        Overwrite(value);
    }

    public void Overwrite(T value)
    {
        if (_size == _capacity)
        {
            _circularBuffer[_start] = value;
            _end = _start;
            _start = (_start + 1) % _capacity;
        }
        else if (_size == 0)
        {
            _circularBuffer[_end] = value;
            _start = _end;
            _size++;
        }
        else
        {
            _circularBuffer[(_end + 1) % _capacity] = value;
            _end = (_end + 1) % _capacity;
            _size++;
        }
    }

    public void Clear()
    {
        _end = 0;
        _start = 0;
        _size = 0;
    }
}