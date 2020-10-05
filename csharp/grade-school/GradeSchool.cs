using System;
using System.Collections.Generic;

public class GradeSchool
{
    private Dictionary<int, List<string>> _gradeSchool =
        new Dictionary<int, List<string>>();

    public void Add(string student, int grade)
    {
        var students = _gradeSchool.GetValueOrDefault(grade, new List<string>());
        students?.Add(student);
        _gradeSchool[grade] = students;
    }

    public IEnumerable<string> Roster()
    {
        var orderedStudents = new List<string>();
        foreach (var (grade, students) in new SortedList<int, List<string>>(_gradeSchool))
        {
           students.Sort();
           foreach (var student in students)
           {
              orderedStudents.Add(student); 
           }
        }

        return orderedStudents.ToArray();
    }

    public IEnumerable<string> Grade(int grade)
    {
        var students = _gradeSchool.GetValueOrDefault(grade, new List<string>());
        students?.Sort();
        return students?.ToArray();
    }
}