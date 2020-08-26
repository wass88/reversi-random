extern crate rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Piece {
    First,
    Second,
    Empty,
}
const SIZE: usize = 8;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reversi {
    board: [[Piece; SIZE]; SIZE],
    first: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Put(usize, usize),
    Pass,
}
impl Action {
    #[allow(dead_code)]
    fn parse(s: &str) -> Result<Action, String> {
        let v: Vec<&str> = s.split(" ").collect();
        match v[0] {
            "put" => Ok(Action::Put(v[1].parse().unwrap(), v[2].parse().unwrap())),
            "pass" => Ok(Action::Pass),
            _ => Err(format!("Unknown played: {:?}", v)),
        }
    }
}
fn is_in_i(y: isize, x: isize) -> bool {
    0 <= y && y < SIZE as isize && 0 <= x && x < SIZE as isize
}
fn is_in(y: usize, x: usize) -> bool {
    y < SIZE && x < SIZE
}
const D8: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];
impl Reversi {
    fn new() -> Reversi {
        use Piece::*;
        let mut board = [[Empty; SIZE]; SIZE];
        board[4][3] = First;
        board[3][4] = First;
        board[3][3] = Second;
        board[4][4] = Second;
        Reversi { board, first: true }
    }
    fn playable(&self) -> Vec<Action> {
        let mut res = vec![];
        for y in 0..SIZE {
            for x in 0..SIZE {
                let r = self.reversal(y, x);
                for &i in &r {
                    if i > 0 {
                        res.push(Action::Put(y, x));
                    }
                }
            }
        }
        if res.len() == 0 {
            res.push(Action::Pass)
        }
        return res;
    }
    fn active(&self) -> Piece {
        if self.first {
            Piece::First
        } else {
            Piece::Second
        }
    }
    fn reversal(&self, y: usize, x: usize) -> [usize; SIZE] {
        use Piece::*;
        assert!(is_in(y, x));
        let s = self.board[y][x];
        let mut res = [0; SIZE];
        if s != Empty {
            return [0; SIZE];
        }
        for (d, (dy, dx)) in D8.iter().enumerate() {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if !is_in_i(ny, nx) {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            let s = self.board[ny][nx];
            if s == self.active() || s == Empty {
                continue;
            }
            for i in 2..SIZE {
                let ny = y as isize + dy * i as isize;
                let nx = x as isize + dx * i as isize;
                if !is_in_i(ny, nx) {
                    break;
                }
                let ny = ny as usize;
                let nx = nx as usize;
                let t = self.board[ny][nx];
                if t == Empty {
                    break;
                }
                if t != s {
                    res[d] = i - 1;
                    break;
                }
            }
        }
        res
    }
    fn is_end(&self) -> bool {
        if self.playable()[0] != Action::Pass {
            return false;
        }
        let b = Reversi {
            board: self.board.clone(),
            first: !self.first,
        };
        b.playable()[0] == Action::Pass
    }
    fn act(&mut self, a: Action) -> Result<(), String> {
        if self.is_end() {
            return Err(format!("game is over"));
        }
        if let Action::Put(y, x) = a {
            if self.board[y][x] != Piece::Empty {
                return Err(format!("({}, {}) is already placed.", y, x));
            }
            let r = self.reversal(y, x);
            if !r.iter().any(|d| d > &0) {
                return Err(format!("No reversing pieces"));
            }
            self.board[y][x] = self.active();
            for (d, n) in r.iter().enumerate() {
                for i in 1..n + 1 {
                    let ny = (y as isize + D8[d].0 * i as isize) as usize;
                    let nx = (x as isize + D8[d].1 * i as isize) as usize;
                    self.board[ny][nx] = self.active()
                }
            }
        } else {
            let a = self.playable();
            if a.len() != 1 || a[0] != Action::Pass {
                return Err(format!("Cannot Pass: {:?}", a));
            }
        }
        self.first = !self.first;
        Ok(())
    }
    fn result(&self) -> isize {
        let mut f = 0;
        let mut s = 0;
        for y in 0..SIZE {
            for x in 0..SIZE {
                match self.board[y][x] {
                    Piece::Empty => {}
                    Piece::First => f += 1,
                    Piece::Second => s += 1,
                }
            }
        }
        if f == 0 {
            -((SIZE * SIZE) as isize)
        } else if s == 0 {
            (SIZE * SIZE) as isize
        } else {
            f - s
        }
    }
}
use std::fmt;
impl fmt::Display for Reversi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = |p| match p {
            Piece::Empty => ".",
            Piece::First => "O",
            Piece::Second => "X",
        };
        let mut s = String::new();
        for y in 0..SIZE {
            for x in 0..SIZE {
                let c = ch(self.board[y][x]);
                s = format!("{}{}", s, c);
            }
            s = format!("{}\n", s);
        }
        if self.is_end() {
            s = format!("{}Over! Result: {}\n", s, self.result());
        } else {
            s = format!("{}{}'s turn\n", s, if self.first { "X" } else { "O" });
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Input {
    Init(usize),
    PlayedPut(usize, usize),
    PlayedPass,
    Res(isize),
    Wait,
}
impl Input {
    fn parse(input: &str) -> Result<Input, String> {
        let v: Vec<&str> = input.split(" ").collect();
        match v[0] {
            "init" => Ok(Input::Init(v[1].parse().unwrap())),
            "played" => match v[1] {
                "put" => Ok(Input::PlayedPut(
                    v[2].parse().unwrap(),
                    v[3].parse().unwrap(),
                )),
                "pass" => Ok(Input::PlayedPass),
                _ => Err(format!("Unknown played: {:?}", v)),
            },
            "res" => Ok(Input::Res(v[1].parse().unwrap())),
            "wait" => Ok(Input::Wait),
            _ => Err(format!("Unknown Input: {:?}", v)),
        }
    }
}
#[derive(Debug, Clone)]
struct RandomPlayer {
    board: Option<Reversi>,
    first: bool,
}
impl RandomPlayer {
    fn new() -> RandomPlayer {
        RandomPlayer {
            board: None,
            first: false,
        }
    }
    fn play(&mut self, input: &str) -> Option<String> {
        let i = Input::parse(input).unwrap();
        match i {
            Input::Init(p) => {
                if p == 0 {
                    self.first = true
                } else {
                    self.first = false
                }
                self.board = Some(Reversi::new());
                None
            }
            Input::PlayedPut(y, x) => {
                let b = self.board.as_mut().unwrap();
                b.act(Action::Put(y, x)).unwrap();
                None
            }
            Input::PlayedPass => {
                let b = self.board.as_mut().unwrap();
                b.act(Action::Pass).unwrap();
                None
            }
            Input::Res(_) => None,
            Input::Wait => {
                use rand::seq::SliceRandom;
                use rand::thread_rng;
                let b = self.board.as_mut().unwrap();
                let p = b.playable();
                let mut rng = thread_rng();
                let a = p.choose(&mut rng).unwrap();
                b.act(*a).unwrap();
                match a {
                    Action::Put(y, x) => Some(format!("put {} {}", y, x)),
                    Action::Pass => Some(format!("pass")),
                }
            }
        }
    }
}
pub fn start() -> Result<(), String> {
    use std::io::{self, BufRead};
    let mut player = RandomPlayer::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let resp = player.play(&line.unwrap());
        if let Some(resp) = resp {
            println!("{}", resp);
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_board() {
        let mut b = Reversi::new();
        let c = b.playable();
        assert_eq!(c.len(), 4);
        let a = vec![(2, 3), (3, 2), (4, 5), (5, 4)];
        fn same_action(e: Vec<Action>, g: Vec<(usize, usize)>) {
            assert_eq!(e.len(), g.len());
            for (y, x) in g {
                assert!(e.iter().any(|a| a == &Action::Put(y, x)))
            }
        }
        same_action(c, a);
        println!("{}", b);
        b.act(Action::Put(2, 3)).unwrap();
        println!("{}", b);
        b.act(Action::Put(2, 2)).unwrap();
        println!("{}", b);
        for _ in 0..SIZE * SIZE {
            if b.is_end() {
                break;
            }
            let c = b.playable();
            b.act(c[0]).unwrap();
        }
        assert!(b.is_end());
        println!("{}", b);
    }
    #[test]
    fn test_player() {
        let mut p0 = RandomPlayer::new();
        let mut p1 = RandomPlayer::new();
        p0.play("init 0");
        p1.play("init 1");
        for _ in 0..100 {
            let r = p0.play("wait").unwrap();
            println!("{:?}", r);
            let p = Action::parse(&r).unwrap();
            p1.board.as_mut().unwrap().act(p).unwrap();
            println!("X\n{}", p0.board.as_mut().unwrap());
            if p1.board.as_mut().unwrap().is_end() {
                break;
            }

            let r = p1.play("wait").unwrap();
            println!("{:?}", r);
            let p = Action::parse(&r).unwrap();
            p0.board.as_mut().unwrap().act(p).unwrap();
            println!("O\n{}", p1.board.as_mut().unwrap());
            if p0.board.as_mut().unwrap().is_end() {
                break;
            }
        }
    }
}
