using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

public static class Tournament
{
    private class Team
    {
        public readonly string Name;
        public int Played;
        public int Won;
        public int Tied;
        public int Lost;
        public int Points;

        public Team(string name, string result)
        {
            Name = name;
            UpdateScores(result);
        }

        public void UpdateScores(string result)
        {
            switch (result)
            {
                case "win":
                    Played += 1;
                    Won += 1;
                    Tied += 0;
                    Lost += 0;
                    Points += 3;
                    break;
                case "loss":
                    Played += 1;
                    Won += 0;
                    Tied += 0;
                    Lost += 1;
                    Points += 0;
                    break;
                case "draw":
                    Played += 1;
                    Won += 0;
                    Tied += 1;
                    Lost += 0;
                    Points += 1;
                    break;
                default:
                    throw new ArgumentException("Invalid result received");
            }
        }
    }

    public static void Tally(Stream inStream, Stream outStream)
    {
        byte[] buffer;
        using (MemoryStream ms = new MemoryStream())
        {
            inStream.CopyTo(ms);
            buffer = ms.ToArray();
        }

        var input = Encoding.UTF8.GetString(buffer);

        var teams = new Dictionary<string, Team>();
        foreach (var line in input.Split(new[] {Environment.NewLine}, StringSplitOptions.None))
        {
            if (line.Length == 0)
            {
                continue;
            }

            var result = line.Split(";").ToList();

            // first team
            if (!teams.ContainsKey(result[0]))
            {
                var team = new Team(result[0], result[2]);
                teams.Add(result[0], team);
            }
            else
            {
                var team = teams[result[0]];
                team.UpdateScores(result[2]);
            }

            // second team
            string secondTeamResult = result[2] switch
            {
                "win" => "loss",
                "loss" => "win",
                "draw" => "draw",
                _ => throw new ArgumentException("Invalid result received")
            };
            if (!teams.ContainsKey(result[1]))
            {
                var team = new Team(result[1], secondTeamResult);
                teams.Add(result[1], team);
            }
            else
            {
                var team = teams[result[1]];
                team.UpdateScores(secondTeamResult);
            }
        }

        var output =
            new StringBuilder($"{"Team",-31}| MP |  W |  D |  L |  P");
        if (teams.Count != 0)
        {
            output.Append("\n");
        }

        foreach (var (index, (_, value)) in teams.OrderByDescending(t => t.Value.Points)
            .ThenBy(t => t.Key).Select((v, i) => (i, v)))
        {
            output.Append(
                $"{value.Name,-31}|  {value.Played} |  {value.Won} |  {value.Tied} |  {value.Lost} |  {value.Points}");
            if (index != teams.Count - 1)
            {
                output.Append("\n");
            }
        }

        outStream.Write(Encoding.Default.GetBytes(output.ToString()));
    }
}