using System;
using System.Collections.Generic;
using System.Linq;
using Microsoft.VisualStudio.TestPlatform.CrossPlatEngine.Helpers;

public class HighScores
{
    private List<int> highScores;

    public HighScores(List<int> list)
    {
        highScores = list;
    }

    public List<int> Scores()
        => highScores;

    public int Latest()
        => highScores[^1];

    public int PersonalBest()
        => PersonalTopThree()[0];

    public List<int> PersonalTopThree() => highScores.OrderByDescending(score => score).ToList()
        .GetRange(0, highScores.Count < 3 ? highScores.Count : 3);
}