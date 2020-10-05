using System;
using System.Collections;
using System.Collections.Generic;

public enum Allergen : int
{
    None = 0,
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}

public class Allergies
{
    private int mask;
    public Allergies(int mask) => this.mask = mask;

    public bool IsAllergicTo(Allergen allergen) => (mask & (int)allergen) != 0;
    public Allergen[] List()
    {
        var allergens = new List<Allergen>();
        for (int i = (int)Allergen.Eggs; i <= (int)Allergen.Cats; i *= 2)
        {
            if ((mask & i) != 0)
            {
                allergens.Add((Allergen)i);
            }
        }

        return allergens.ToArray();
    }
}