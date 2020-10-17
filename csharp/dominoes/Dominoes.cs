using System;
using System.Collections.Generic;
using System.Linq;

public static class Dominoes
{
    public static bool CanChainImpl(List<(int, int)> dominoes,
        List<(int, int)> chain, List<int> indexes)
    {
        if (dominoes.Count == 0 ||
            (chain.Count == dominoes.Count && chain[^1].Item2 == chain[0].Item1) ||
            (chain.Count == dominoes.Count && chain.Count == 1 && chain[^1].Item1 == chain[^1].Item2))
        {
            return true;
        }
        
        foreach (var (i,p) in dominoes.Select((v, i) => (i, v)))
        {
            foreach (var d in new List<(int, int)> {p, (p.Item2, p.Item1)})
            {
                if ((chain.Contains(d) || chain.Contains((d.Item2, d.Item1)) ||
                     (chain.Count != 0 && chain[^1].Item2 != d.Item1)) &&
                    (indexes.Contains(i) || chain[^1].Item2 != d.Item1))
                {
                    continue;
                }

                chain.Add(d);
                indexes.Add(i);
                if (CanChainImpl(dominoes, chain, indexes))
                {
                    return true;
                }
                chain.Remove(d);
                indexes.Remove(i);
            }
        }

        return false;
    }

    public static bool CanChain(IEnumerable<(int, int)> dominoes)
    {
        var chain = new List<(int, int)>();
        var indexes = new List<int>();
        return CanChainImpl(dominoes.ToList(), chain, indexes);
    }
}