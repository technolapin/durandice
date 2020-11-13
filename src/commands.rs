#[derive(Debug)]
pub enum Command
{
    Roll(i64, i64),
    Warhammer
    {
        score: i64,
    },
    Pouet,
    Coupable,
    Shadowrun(i64, Option<i64>),
    Degenesis(i64, Option<i64>),
    Trudvang(i64, i64, i64),
    Help,
    Say(String),
    Brigandine
    {
        explode_tresh: i64,
    },
    CmdList(Vec<Self>),
    CommentedCmd(String, Box<Self>),
}

pub fn dice(n_faces: i64) -> i64
{
    (rand::random::<u64>() % (n_faces as u64) + 1) as i64
}

fn explode<F>(n_faces: i64,
              condition: F,
              depth: usize) -> Result<Vec<i64>, String>
where
    F: Fn(i64)-> bool
{
    if depth > 1000
    {
        return Err(format!("nan mais t'abuse là avec ton dé explosif"))
    }
    let r = dice(n_faces);
    let mut v = vec![r];
    if !condition(r)
    {
        Ok(v)
    }
    else
    {
        v.append(&mut explode(n_faces, condition, depth+1)?);
        Ok(v)
    }
}

impl Command
{
    pub fn execute(self) -> String
    {
        match self
        {
            Self::Help =>
            {
                String::from(r#"
#Syntaxe
Les espaces sont ignorés, mettez en autant que vous voulez (ou pas du tout)
Un paramètre du type <n> est obligatoire
Un paramètre du type [n] est optionel

## Commande commentée
'commentaire': <n'importe quelle autre commande>

## liste de commandes
<commande 1> ; <commande 2> ; ...

## Dés normaux
commande: "d" ou "roll"

"?[nb de dés]d<nb de faces>"

Exemples:

?4d100
?d100
?10000d0 => oui bon

## Jets de warhammer v4
commande: "warhammer" OU "wh"

"?wh <score>"

Exemples:

?wh 64 => un jet sous 64
?wh 100 => un jet de CC à -30 pour Rudolf

## Shadowrun
commande: "sr" OU "shadowrun"

"?sr <score> [difficulté]"

Exemple:

?sr 10 => un jet à 10 dés
?sr 10 3 => un jet à 10 dés où on doit atteindre 3 réussites

## Degenesis
commande: "dg" OU "dege" OU "degenesis"

"?dg <score> [difficulté]"

Exemple:

?dg 10 => un jet à 10 dés
?dg 17 => un jet à 12 dés plus 5 réussites automatiques
?dg 5 3 => un jet à 5 dés où on doit atteindre 3 réussites

## Trudvang
commande: "trv" OU "trudvang" OU "trud"

"?trv[score] [JO <seuil>] [bonus/malus/bon/mal <bonus ou malus>]"

Exemples:

?trv => 1d10 tout simple qui n'explose pas
?trv 3 => 3d10 qui n'explosent pas
?trv JO 10 => 1d10 qui explose sur 10
?trv 4 JO 5 => 4d10 qui explosent sur 5 ou plus
?trv JO 0 => la mort du respect
?trv 4 JO 5 BON 2=> 4d10 qui explosent sur 5 ou plus avec un bonus de 2
?trv 4 JO 5 MAL 2=> 4d10 qui explosent sur 5 ou plus avec un malus de 2


## Brigandine
commande: "brig" ou "br" ou "brigandine"

"?br [seuil d'explosion]"

Exemples:

?br => jet de brigandine qui explose sur les 10
?br 9 => jet de brigandine qui explose sur 9+ 
?br 6 => jet de brigandine qui explose sur 6+ 
?br 1 => non

"#)
            },
            Self::Roll(sides, n) =>
            {
                let mut out = String::new();
                for _ in 0..n
                {
                    out = format!("{} {}", out, dice(sides));
                }
                out
            },
            Self::Pouet => String::from("Pouet!"),
            Self::Warhammer{score} =>
            {
                let d = dice(100);
                if d % 11 == 0
                {
                    if d <= score
                    {
                        format!("{}: Réussite critique de {} degrés", d, score/10 - d/10)
                    }
                    else
                    {
                        format!("{}: Échec critique de {} degrés", d, d/10 - score/10)
                    }
                    
                }
                else
                {
                    if d <= score
                    {
                        format!("{}: Réussite de {} degrés", d, score/10 - d/10)
                    }
                    else
                    {
                        format!("{}: Échec de {} degrés", d, d/10 - score/10)
                    }

                }
            },
            Self::Coupable =>
            {
                format!("Jovial.")
            },
            Self::Shadowrun(n, maybe_goal) =>
            {
                let mut dies = (0..n).map(|_| dice(6)).collect::<Vec<_>>();
                dies.sort();

                let n_success = dies.iter().filter(|&&a| a >= 5).count() as i64;
                let n_ones =  dies.iter().filter(|&&a| a == 1).count() as i64;

                let throw_s = dies.iter()
                    .fold(String::new(), |s, n| format!("{} {}", s, n));

                let complication_m = if n_ones > n_success
                {format!("Complication - ")}
                else
                {String::new()};
                
                let msg = if let Some(goal) = maybe_goal
                {
                    if goal <= n_success
                    {
                        format!("Réussite ({}/{})", n_success, goal)
                    }
                    else if n_ones > n_success
                    {
                        format!("Échec critique! ({}/{})", n_success, goal)
                    }
                    else
                    {
                        format!("Échec ({}/{})", n_success, goal)
                    }
                }
                else
                {
                    if n_ones > n_success
                    {
                        format!("{} réussites (Risque d'échec critique)", n_success)
                    }
                    else
                    {
                        format!("{} réussites", n_success)
                    }

                };

                format!("[{}]\n{}{}", throw_s, complication_m, msg)
                
            },
            Self::Degenesis(n, maybe_goal) =>
            {
                let mut dies = (0..n.min(12)).map(|_| dice(6)).collect::<Vec<_>>();
                dies.sort();
                let n_auto = n - dies.len() as i64;
                let n_success = dies.iter().filter(|&&a| a >= 4).count() as i64 + n_auto;
                let n_trigg = dies.iter().filter(|&&a| a == 6).count() as i64;
                let n_ones =  dies.iter().filter(|&&a| a == 1).count() as i64;

                let throw_s = dies.iter()
                    .fold(String::new(), |s, n| format!("{} {}", s, n));

                let m_auto = if n_auto == 0 {String::new()} else {format!("({} automatiques) ", n_auto)};
                let m_triggers = if n_trigg == 0 {String::new()} else {format!("dont {} triggers", n_trigg)};
                let m_bilan = format!("{} réussites {}{}", n_success, m_auto, m_triggers);
                let m_analyse = if let Some(goal) = maybe_goal
                {
                    if n_success >= goal
                    {
                        format!("Réussite ({} sur {})", n_success, goal)
                    }
                    else if n_ones > n_success
                    {
                        format!("Échec critique!")
                    }
                    else
                    {
                        format!("Échec")
                    }

                }
                else
                {
                    if n_ones > n_success
                    {
                        format!("Possibilité d'échec critique")
                    }
                    else
                    {
                        format!("")
                    }
                    
                };
                
                format!("[{}]\n{}\n{}", throw_s, m_bilan, m_analyse)
                
                
            },
            Self::Trudvang(n, expl_tresh, bonus) =>
            {
                let mut dices = Vec::new();
                dices.resize_with(
                    n as usize,
                    || {explode(10, |n| n >= expl_tresh, 0)}
                );

                if let Some(Err(err)) = dices.iter()
                    .find(|maybe| maybe.is_err())
                {
                    format!("{}", err)
                }
                else
                {
                    let dices = dices.iter().map(|maybe| maybe.clone().unwrap()).collect::<Vec<_>>();
                    let sum = dices.iter().flatten()
                        .fold(0i64, |sum, die| sum + die) + bonus;
                    let throw_m = dices.iter()
                        .map(|v|
                             {
                                 v.iter()
                                     .fold(String::new(),
                                           |s, n| format!("{} {}", s, n))
                             }
                        )
                        .fold(String::new(),
                              |out, s| format!("[{}] {}", s, out)
                        );
                    let total_m = format!("Total: {}", sum);
                    format!("{}\n{}", throw_m, total_m)

                }
            },
            Self::Say(s) =>
            {
                s.clone()
            },
            Self::Brigandine{explode_tresh} =>
            {
                let mut units = vec![];
                match explode(10, |n| n >= explode_tresh, 0)
                {
                    Err(err) => {return err;},
                    Ok(mut explosion) =>
                    {
                        units.append(
                            &mut explosion
                        );
                    }
                }

                
                let unit_dice = units[0];
                let tenth_dice = dice(10);
                let d100 = (tenth_dice % 10)*10 + (unit_dice % 10);
                let inverse = (unit_dice % 10)*10 + (tenth_dice % 10);
                println!("inverse: {}", inverse);
                let hit_location = match inverse
                {
                    1..=9 => "Tête",
                    10 => "Main gauche",
                    11..=24 => "Bras gauche",
                    25 => "Main droite",
                    26..=44 => "Bras droit",
                    45..=69 => "Torse",
                    70..=80 => "Abdomen",
                    81..=88 => "Jambe gauche",
                    89 => "Pied gauche",
                    90..=99 => "Jambe droite",
                    100 => "Pied droit",
                    _ => unreachable!()
                };
                

                let damages: i64 = units.iter().sum();
                
                let throw_m = units[1..].iter()
                    .fold(format!("{}", units[0]), |s, n| format!("{}+{}", s, n));
                let explode_m = if units.len() == 1
                {String::new()}
                else
                {
                    if dice(100) == 1
                    {
                        format!("(explosion: {}) Macron Explosion!", throw_m)
                    }
                    else
                    {
                        format!("(explosion: {})", throw_m)
                    }
                };
                let location_m = format!("localisation: {}", hit_location);
                format!("test: {} dégâts: {}  {}\n{}", d100, damages, explode_m, location_m)
                
                
            },

            Self::CmdList(commands) =>
            {
               commands.into_iter()
                    .fold(format!(""),
                          |s, cmd| format!("{}===================================\n{}\n", s, cmd.execute())
                    )
            },
            Self::CommentedCmd(com, cmd) =>
            {
                format!("##{}\n{}", com, (*cmd).execute())
            }
        }
    }
}
