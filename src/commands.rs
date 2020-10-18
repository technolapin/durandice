#[derive(Debug)]
pub enum Command
{
    Roll(u64, u64),
    Warhammer
    {
        score: u64,
    },
    Pouet
}

fn dice(n_faces: u64) -> u64
{
    rand::random::<u64>() % n_faces + 1
}

impl Command
{
    pub fn execute(self) -> String
    {
        match self
        {
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
                if d <= score
                {
                    format!("{}: Réussite de {} degrés", d, score/10 - d/10)
                }
                else
                {
                    format!("{}: Échec de {} degrés", d, d/10 - score/10)
                }
            }
        }
    }
}
