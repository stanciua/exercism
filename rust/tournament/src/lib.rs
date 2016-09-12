#![feature(slice_patterns)]
use std::collections::HashMap;
use std::fmt;
use std::cmp::{Ordering, Ord};

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
pub struct Team<'a> {
    points: u8,
    name: &'a str,
    matches: u8,
    wins: u8,
    draws: u8,
    losses: u8,
}

impl<'a> fmt::Display for Team<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:31}|  {} |  {} |  {} |  {} |  {}",
               self.name,
               self.matches,
               self.wins,
               self.draws,
               self.losses,
               self.points)
    }
}

pub fn tally(input: &String) -> String {
    let match_rslt = vec!["win", "loss", "draw"];
    let teams_rslts = input.split('\n')
        .map(|line| line.split(';').skip_while(|val| val.is_empty()))
        .map(|splits| splits.collect::<Vec<_>>())
        .filter(|elems| elems.len() == 3 && match_rslt.contains(elems.last().unwrap()))
        .collect::<Vec<_>>();

    let mut team_standings: HashMap<&str, Team> = HashMap::new();

    for rslt in teams_rslts {
        match rslt.as_slice() {
            &[team1, team2, "win"] => {
                {
                    let t1 = team_standings.entry(team1).or_insert(Team {
                        name: team1,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t1.matches += 1;
                    t1.wins += 1;
                    t1.points += 3;
                }
                {
                    let t2 = team_standings.entry(team2).or_insert(Team {
                        name: team2,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t2.matches += 1;
                    t2.losses += 1;
                }
            }
            &[team1, team2, "draw"] => {

                {
                    let t1 = team_standings.entry(team1).or_insert(Team {
                        name: team1,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t1.matches += 1;
                    t1.draws += 1;
                    t1.points += 1;
                }
                {
                    let t2 = team_standings.entry(team2).or_insert(Team {
                        name: team2,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t2.matches += 1;
                    t2.draws += 1;
                    t2.points += 1;
                }
            }
            &[team1, team2, "loss"] => {
                {
                    let t1 = team_standings.entry(team1).or_insert(Team {
                        name: team1,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t1.matches += 1;
                    t1.losses += 1;
                }
                {
                    let t2 = team_standings.entry(team2).or_insert(Team {
                        name: team2,
                        matches: 0,
                        wins: 0,
                        draws: 0,
                        losses: 0,
                        points: 0,
                    });
                    t2.matches += 1;
                    t2.wins += 1;
                    t2.points += 3;
                }
            }
            _ => {}
        }
    }

    let mut standings = team_standings.values().collect::<Vec<_>>();
    standings.sort_by(|t1, t2| {
        match t1.points.cmp(&t2.points) {
            Ordering::Equal => t1.name.cmp(&t2.name),
            order => order.reverse(),
        }
    });

    let mut output = "Team                           | MP |  W |  D |  L |  P\n".to_string();
    output.extend(standings.into_iter()
        .map(|team| format!("{}", team))
        .collect::<Vec<_>>()
        .join("\n")
        .chars());
    output



}
