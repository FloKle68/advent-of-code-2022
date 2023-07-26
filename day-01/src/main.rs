use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    color_eyre::install()?;

    let input = fs::read_to_string("input.txt")?;

    let solution_partie_une = partie_une(input.as_str());
    let solution_partie_deux = partie_deux(input.as_str());

    println!("Solution part 1 : {solution_partie_une}");
    println!("Solution part 2 : {solution_partie_deux}");

    Ok(())
}

fn partie_une(input: &str) -> u32 {
    let resultat = input
        .split("\n\n")
        .map(|charge| {
            charge
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    resultat
}

fn partie_deux(input: &str) -> u32 {
    let mut liste_charge = input
        .split("\n\n")
        .map(|charge| {
            charge
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

        liste_charge.sort();

    return liste_charge.into_iter().rev().take(3).sum::<u32>();
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_partie_une() {
        assert_eq!(partie_une(INPUT), 24000);
    }

    #[test]
    fn test_partie_deux() {
        assert_eq!(partie_deux(INPUT), 45000);
    }
}
