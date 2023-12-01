#[allow(unused)] use aoc::prelude::*;

#[derive(Default)]
struct MultiSlice<'a> {
    parts: Vec<&'a str>,
}

impl<'a> MultiSlice<'a> {
    fn new(parts: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            parts: parts.into_iter().filter(|part| !part.is_empty()).collect(),
        }
    }

    fn char_indices(&self) -> impl Iterator<Item = ([usize; 2], char)> + '_ {
        self.parts.iter()
            .enumerate()
            .flat_map(|(part_idx, part)| part.char_indices().map(move |(char_idx, c)| ([part_idx, char_idx], c)))
    }

    fn split_at(&self, [part_idx, char_idx]: [usize; 2]) -> (Self, Self) {
        let (parts_prefix, parts_suffix) = self.parts.split_at(part_idx);
        let (part, parts_suffix) = parts_suffix.split_first().unwrap();
        let (prefix, suffix) = part.split_at(char_idx);
        (
            Self::new(parts_prefix.iter().copied().chain(iter::once(prefix))),
            Self::new(iter::once(suffix).chain(parts_suffix.iter().copied())),
        )
    }

    fn strip_prefix(&self, prefix: &str) -> Option<Self> {
        let mut char_indices = self.char_indices();
        for c in prefix.chars() {
            if char_indices.next()?.1 != c {
                return None
            }
        }
        Some(if let Some(([part_idx, char_idx], _)) = char_indices.next() {
            let (part, parts_suffix) = self.parts[part_idx..].split_first().unwrap();
            Self::new(iter::once(&part[char_idx..]).chain(parts_suffix.iter().copied()))
        } else {
            Self::default()
        })
    }
}

impl<'a> PartialEq for MultiSlice<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.parts.iter().flat_map(|part| part.bytes()).eq(other.parts.iter().flat_map(|part| part.bytes()))
    }
}

impl<'a> Eq for MultiSlice<'a> {}

impl<'a> Hash for MultiSlice<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for part in &self.parts {
            state.write(part.as_ref());
        }
        state.write_u8(0xff);
    }
}

fn products<'a>(replacements: &'a [[String; 2]], input: &'a str) -> HashSet<MultiSlice<'a>> {
    let mut products = HashSet::default();
    for [from, to] in replacements {
        for (idx, _) in input.char_indices() {
            let (prefix, suffix) = input.split_at(idx);
            if let Some(suffix) = suffix.strip_prefix(from) {
                products.insert(MultiSlice::new([prefix, to, suffix]));
            }
        }
    }
    products
}

fn products_rev<'a, 'b>(replacements: &'a [[String; 2]], input: &'b MultiSlice<'a>) -> impl ParallelIterator<Item = MultiSlice<'a>> + 'b {
    replacements.par_iter()
        .flat_map_iter(|[from, to]| input.char_indices().filter_map(|(idx, _)| {
            let (prefix, suffix) = input.split_at(idx);
            if let Some(suffix) = suffix.strip_prefix(to) {
                Some(MultiSlice::new(prefix.parts.iter().copied().chain(iter::once(&**from)).chain(suffix.parts.iter().copied())))
            } else {
                None
            }
        }))
}

#[aoc_generator(day19)]
fn gen(input: &str) -> (Vec<[String; 2]>, String) {
    let (replacements, medicine) = input.rsplit_once("\n\n").unwrap();
    (
        replacements.lines().map(|line| {
            let (from, to) = line.split_once(" => ").unwrap();
            [from.to_owned(), to.to_owned()]
        }).collect(),
        medicine.to_owned(),
    )
}

#[aoc(day19, part1)]
fn part1((replacements, medicine): &(Vec<[String; 2]>, String)) -> usize {
    products(replacements, medicine).len()
}

#[aoc(day19, part2)]
fn part2((replacements, medicine): &(Vec<[String; 2]>, String)) -> usize {
    let electron = MultiSlice::new(iter::once("e"));
    let mut stack = vec![vec![MultiSlice::new(iter::once(&**medicine))]];
    loop {
        if let Some(input) = stack.last_mut().unwrap().pop() {
            let products_rev = products_rev(replacements, &input).collect::<HashSet<_>>();
            if products_rev.contains(&electron) {
                break stack.len()
            }
            stack.push(products_rev.into_iter().collect());
        } else {
            stack.pop();
        }
    }
}
