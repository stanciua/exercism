using System;
using System.Collections.Generic;
using System.Text;
using static System.String;

public class Robot
{
    private static readonly HashSet<string> Registry = new HashSet<string>();
    private static readonly Random Generator = new Random();
    private string _name = Empty;

    public string Name
    {
        get
        {
            if (this._name.Length != 0)
            {
                return this._name;
            }

            Name = generateId();
            // we need to make sure that the name is unique using the registry
            while (Registry.Contains(this._name))
            {
                Name = generateId();
            }

            Registry.Add(this._name);

            return this._name;
        }
        private set => this._name = value;
    }

    public void Reset()
    {
        Registry.Remove(Name);
        Name = Empty;
    }

    private string generateId()
    {
        const int offset = 26;
        // 1st uppercase letter
        char c1 = (char)Robot.Generator.Next('A', 'A' + offset);
        // 2nd uppercase letter
        char c2 = (char)Robot.Generator.Next('A', 'A' + offset);
        // 3 random digits
        int digits3 = Robot.Generator.Next(100, 999);
        var id = new StringBuilder();
        return id.Append(c1).Append(c2).Append(digits3).ToString();
    }
}