#![cfg_attr(all(test, feature = "unstable"), feature(test))]

extern crate rand;
extern crate heap;

use std::fmt::{self, Display, Formatter};
use std::collections::HashSet;
use std::cmp::{PartialOrd, Ordering};

use heap::Heap;

use Dir::{Up, Down, Left, Right};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Puzzle {
    state: [u8; 9]
}

impl Puzzle {
    #[allow(dead_code)]
    fn from_state(state: [u8; 9]) -> Self {
        Puzzle { state: state }
    }

    fn rand_puzzle() -> Self {
        let mut state = [0, 1, 2, 3, 4, 5, 6, 7, 8];

        for i in (0..state.len()).rev() {
            let pos = rand::random::<u8>() % (i + 1) as u8;
            state.swap(i, pos as usize);
        }

        Puzzle { state: state }
    }

    fn is_solved(&self) -> bool {
        for (i, num) in self.state.iter().enumerate() {
            if i as u8 != *num {
                return false;
            }
        }
        true
    }

    fn child_states<'a>(&'a self) -> PuzzleIter<'a> {
        PuzzleIter::new(self)
    }

    fn heuristic(&self) -> u8 {
        let mut cost = 0;
        for (i, num) in self.state.iter().enumerate() {
            let (x1, y1) = ((*num / 3) as i8, (*num % 3) as i8);
            let (x2, y2) = ((i / 3) as i8, (i % 3) as i8);
            cost += (x2 - x1).abs() + (y2 - y1).abs();
        }
        cost as u8
    }

    fn move_(mut self, dir: Dir, index: u8) -> Self {
        let index = index as usize;
        assert!(index < self.state.len());
        assert_eq!(self.state[index], 8);
        let (y, x) = ((index / 3), (index % 3));
        match dir {
            Up => {
                self.state.swap(index, (y - 1) * 3 + x);
            }
            Down => {
                assert!(y + 1 < 3);
                self.state.swap(index, (y + 1) * 3 + x);
            }
            Left => {
                self.state.swap(index, y * 3 + x - 1);
            }
            Right => {
                assert!(x + 1 < 3);
                self.state.swap(index, y * 3 + x + 1);
            }
        }
        self
    }

    fn move__(self, dir: Dir) -> Self {
        let index = self.state.iter().position(|x| *x == 8).expect("9 not found in state");
        self.move_(dir, index as u8)
    }
}

#[derive(PartialEq, Eq)]
struct QueueElt {
    puzzle: Puzzle,
    f: usize,
    path: Vec<Dir>
}

impl QueueElt {
    fn new(puzzle: Puzzle, path: Vec<Dir>) -> Self {
        QueueElt { f: path.len() + puzzle.heuristic() as usize, puzzle: puzzle, path: path }
    }
}

impl PartialOrd for QueueElt {
    fn partial_cmp(&self, other: &QueueElt) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

impl Ord for QueueElt {
    fn cmp(&self, other: &QueueElt) -> Ordering {
        other.f.cmp(&self.f)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Dir {
    Up, Down, Left, Right
}

struct PuzzleIter<'a> {
    puzzle: &'a Puzzle,
    dir: Option<Dir>,
    index: u8
}

impl<'a> PuzzleIter<'a> {
    fn new(puzzle: &'a Puzzle) -> Self {
        let index = puzzle.state.iter().position(|x| *x == 8).expect("9 not found in state");
        PuzzleIter { puzzle: puzzle, dir: Some(Up), index: index as u8 }
    }
}

impl<'a> Iterator for PuzzleIter<'a> {
    type Item = (Puzzle, Dir);

    fn next(&mut self) -> Option<(Puzzle, Dir)> {
        match self.dir {
            Some(dir) => {
                match dir {
                    Up => {
                        self.dir = Some(Down);
                        if self.index / 3 == 0 {
                            self.next()
                        } else {
                            Some((self.puzzle.clone().move_(Up, self.index), Up))
                        }
                    }
                    Down => {
                        self.dir = Some(Left);
                        if self.index / 3 == 2 {
                            self.next()
                        } else {
                            Some((self.puzzle.clone().move_(Down, self.index), Down))
                        }
                    }
                    Left => {
                        self.dir = Some(Right);
                        if self.index % 3 == 0 {
                            self.next()
                        } else {
                            Some((self.puzzle.clone().move_(Left, self.index), Left))
                        }
                    }
                    Right => {
                        self.dir = None;
                         if self.index % 3 == 2 {
                            self.next()
                        } else {
                            Some((self.puzzle.clone().move_(Right, self.index), Right))
                        }
                    }
                }
            }
            None => None
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}\n{} {} {}\n{} {} {}",
               self.state[0], self.state[1], self.state[2],
               self.state[3], self.state[4], self.state[5],
               self.state[6], self.state[7], self.state[8])
    }
}

#[cfg(not(test))]
fn main() {
    let puzzle = Puzzle::rand_puzzle();
    println!("Initial state:\n{}", puzzle);

    println!("Solving...");
    match solve(puzzle.clone()) {
        Some(solution) => {
            println!("Solution: {:?}", solution);
            println!("Path length: {}", solution.len());
            simulate(puzzle, solution);
        }
        None => println!("No solution found")
    }
}

fn solve(puzzle: Puzzle) -> Option<Vec<Dir>> {
    let mut visited = HashSet::new();
    let mut queue = Heap::new();
    queue.push(QueueElt::new(puzzle, Vec::new()));
    while let Some(QueueElt { puzzle, path, .. }) = queue.pop() {
        if puzzle.is_solved() {
            return Some(path);
        }

        for (p, dir) in puzzle.child_states() {
            if !visited.contains(&p) {
                let mut path = path.clone();
                path.push(dir);
                queue.push(QueueElt::new(p, path));
            }
        }
        visited.insert(puzzle);
    }
    None
}

#[cfg(not(test))]
fn simulate(mut puzzle: Puzzle, moves: Vec<Dir>) {
    println!("{}", puzzle);
    for dir in moves {
        puzzle = puzzle.move__(dir);
        println!("{:?}", dir);
        println!("{}", puzzle);
    }
}

#[cfg(test)]
mod test {
    use super::Puzzle;
    use super::Dir::{Up, Down, Left, Right};

    #[test]
    fn test_heuristic() {
        let p = Puzzle::from_state([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(p.heuristic(), 0);

        let p = Puzzle::from_state([1, 0, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(p.heuristic(), 2);

        let p = Puzzle::from_state([8, 1, 2, 3, 4, 5, 6, 7, 0]);
        assert_eq!(p.heuristic(), 8);

        let p = Puzzle::from_state([0, 1, 2, 5, 4, 3, 6, 7, 8]);
        assert_eq!(p.heuristic(), 4);
    }

    #[test]
    fn test_iterator() {
        let p = Puzzle::from_state([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Left);

        let p = Puzzle::from_state([0, 1, 2, 3, 4, 5, 6, 8, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Left);
        assert_eq!(it.next().unwrap().1, Right);

        let p = Puzzle::from_state([0, 1, 2, 3, 4, 5, 8, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Right);

        let p = Puzzle::from_state([0, 1, 2, 3, 4, 8, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Left);

        let p = Puzzle::from_state([0, 1, 2, 3, 8, 4, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Left);
        assert_eq!(it.next().unwrap().1, Right);

        let p = Puzzle::from_state([0, 1, 2, 8, 3, 4, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Up);
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Right);

        let p = Puzzle::from_state([0, 1, 8, 2, 3, 4, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Left);

        let p = Puzzle::from_state([0, 8, 1, 2, 3, 4, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Left);
        assert_eq!(it.next().unwrap().1, Right);

        let p = Puzzle::from_state([8, 0, 1, 2, 3, 4, 5, 6, 7]);
        let mut it = p.child_states();
        assert_eq!(it.next().unwrap().1, Down);
        assert_eq!(it.next().unwrap().1, Right);
    }

    #[cfg(feature = "unstable")]
    mod bench {
        extern crate test;
        use self::test::Bencher;

        use {Puzzle, solve};

        #[bench]
        fn bench1(b: &mut Bencher) {
            let p = Puzzle::from_state([0, 7, 8, 3, 6, 2, 5, 4, 1]);
            b.iter(|| {
                solve(p.clone());
            });
        }

        #[bench]
        fn bench_unsolvable(b: &mut Bencher) {
            let p = Puzzle::from_state([0, 1, 2, 5, 4, 3, 6, 7, 8]);
            b.iter(|| {
                solve(p.clone());
            });
        }
    }
}
