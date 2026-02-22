enum SearchIterator<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Iterator for SearchIterator<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Left(it) => it.next(),
            Self::Right(it) => it.next(),
        }
    }
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    case_sensitive: bool,
) -> impl Iterator<Item = &'a str> {
    if case_sensitive {
        SearchIterator::Left(contents.lines().filter(move |line| line.contains(query)))
    } else {
        // Optimization: Pre-lowercase once
        let lower_query = query.to_lowercase();

        SearchIterator::Right(
            contents
                .lines()
                .filter(move |line| line.to_lowercase().contains(&lower_query)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let results: Vec<&str> = search(query, contents, true).collect();

        assert_eq!(vec!["safe, fast, productive."], results);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let results: Vec<&str> = search(query, contents, false).collect();
        assert_eq!(vec!["Rust:", "Trust me."], results);
    }
}
