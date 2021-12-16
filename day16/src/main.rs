use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Version(u8);

#[derive(Debug, PartialEq, Clone, Copy)]
struct TypeId(u8);

#[derive(Debug, Clone, Copy)]
struct Header {
    v: Version,
    t: TypeId,
}

#[derive(Debug)]
enum Packet {
    Literal(Header, u64),
    Operator(Header, Vec<Packet>),
}

struct Cursor<'a> {
    src: &'a str,
    idx: usize,
}

impl<'a> Cursor<'a> {
    fn read_bits(&mut self, n: usize) -> Result<String, String> {
        if self.idx + n > self.src.len() {
            let err = format!(
                "Cursor index oob. idx={}, n={}, s={}, s.len()={}",
                self.idx,
                n,
                self.src,
                self.src.len()
            );
            return Err(err);
        }

        let rv = self.src[self.idx..self.idx + n].chars().collect::<Vec<_>>();
        self.idx += n;

        Ok(rv.into_iter().collect())
    }

    fn read_header(&mut self) -> Result<Header, String> {
        let h = self.read_bits(6)?.parse::<Header>()?;
        Ok(h)
    }

    fn read_literal_val(&mut self) -> Result<u64, String> {
        let mut buf = vec![];
        loop {
            let five_chars = self.read_bits(5)?;
            buf.extend(five_chars.chars().skip(1));

            if five_chars.starts_with('0') {
                break;
            }
        }
        let buf = buf.into_iter().collect::<String>();
        let n = u64::from_str_radix(&buf, 2).expect(&format!("buf={:?}", buf));
        Ok(n)
    }

    fn read_operators(&mut self) -> Result<Vec<Packet>, String> {
        let length_type_id = self.read_bits(1)?;
        match &length_type_id[..] {
            "0" => {
                let nbits_bin = self.read_bits(15)?;
                let nbits = usize::from_str_radix(&nbits_bin, 2).unwrap();

                let ps = self.parse_seq_nbits(nbits)?;
                Ok(ps)
            }

            "1" => {
                let npackets_bin = self.read_bits(11)?;
                let npackets = usize::from_str_radix(&npackets_bin, 2).unwrap();

                let ps = self.parse_seq_npackets(npackets)?;

                Ok(ps)
            }

            _ => panic!(),
        }
    }

    fn parse_recursive(&mut self) -> Result<Packet, String> {
        let h = self.read_header()?;
        match &h {
            h if h.t == TypeId(4) => {
                let p = Packet::Literal {
                    0: *h,
                    1: self.read_literal_val()?,
                };

                Ok(p)
            }

            h => {
                let p = Packet::Operator {
                    0: *h,
                    1: self.read_operators()?,
                };

                Ok(p)
            }
        }
    }

    fn parse_seq_nbits(&mut self, nbits: usize) -> Result<Vec<Packet>, String> {
        let mut rv = vec![];
        let end_idx = self.idx + nbits;

        loop {
            let pocket = self.parse_recursive()?;
            rv.push(pocket);

            if self.idx >= end_idx {
                break;
            }
        }

        Ok(rv)
    }

    fn parse_seq_npackets(&mut self, npackets: usize) -> Result<Vec<Packet>, String> {
        let mut rv = vec![];

        for _ in 0..npackets {
            let pocket = self.parse_recursive()?;
            rv.push(pocket);
        }

        Ok(rv)
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim()
            .chars()
            .map(|c| u8::from_str_radix(&c.to_string(), 16).unwrap())
            .flat_map(|n| format!("{:0>4b}", n).chars().collect::<Vec<_>>())
            .collect::<String>();

        let mut cursor = Cursor { src: &s, idx: 0 };
        cursor.parse_recursive()
    }
}

impl FromStr for Header {
    type Err = String;
    fn from_str(s: &str) -> Result<Header, String> {
        let vs = s.chars().take(3).collect::<String>();
        let v = u8::from_str_radix(&vs, 2).expect("can't convert");

        let ts = s.chars().skip(3).take(3).collect::<String>();
        let t = u8::from_str_radix(&ts, 2).expect("can't convert.");

        Ok(Header {
            v: Version(v),
            t: TypeId(t),
        })
    }
}

fn version_sum(p: &Packet) -> u32 {
    match p {
        Packet::Literal(h, _) => h.v.0 as u32,

        Packet::Operator(h, v) => (h.v.0 as u32) + v.iter().map(|p| version_sum(p)).sum::<u32>(),
    }
}

fn eval(p: &Packet) -> u64 {
    match p {
        Packet::Literal(l, v) => *v,

        Packet::Operator(h, v) => match h.t {
            TypeId(0) => v.iter().map(|p| eval(p)).sum::<u64>(),
            TypeId(1) => v.iter().map(|p| eval(p)).product::<u64>(),
            TypeId(2) => v.iter().map(|p| eval(p)).min().unwrap(),
            TypeId(3) => v.iter().map(|p| eval(p)).max().unwrap(),

            TypeId(5) => {
                let fst = eval(&v[0]);
                let snd = eval(&v[1]);

                if fst > snd {
                    1
                } else {
                    0
                }
            }

            TypeId(6) => {
                let fst = eval(&v[0]);
                let snd = eval(&v[1]);

                if fst < snd {
                    1
                } else {
                    0
                }
            }

            TypeId(7) => {
                let fst = eval(&v[0]);
                let snd = eval(&v[1]);

                if fst == snd {
                    1
                } else {
                    0
                }
            }

            _ => panic!(),
        },
    }
}

fn inner_main() -> Result<(), String> {
    let input = std::fs::read_to_string("aoc16.txt").unwrap();

    let p1 = input
        .trim()
        .lines()
        .map(|line| line.trim().parse::<Packet>().unwrap())
        .map(|p| version_sum(&p))
        .sum::<u32>();
    println!("P1: {}", p1);

    let p2 = eval(&input.trim().parse::<Packet>()?);
    println!("P2: {}", p2);

    Ok(())
}

fn main() {
    if let Err(msg) = inner_main() {
        println!("Error: {}", msg);
    }
}
