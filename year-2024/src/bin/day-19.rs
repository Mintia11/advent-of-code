#[derive(Clone)]
struct Input {
    possible: Trie,
    towels: Vec<String>,
}

#[derive(Debug, Clone)]
struct Node {
    children: [usize; 5],
    terminal: bool,
}

#[derive(Debug, Clone)]
struct Trie {
    nodes: Vec<Node>,
}

#[inline]
pub const fn char_to_idx(chr: u8) -> usize {
    match chr {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => unreachable!(),
    }
}

impl Trie {
    pub fn new() -> Self {
        let mut nodes = Vec::with_capacity(1500);
        nodes.insert(
            0,
            Node {
                children: [0; 5],
                terminal: false,
            },
        );

        Self { nodes }
    }

    fn push_child(&mut self, terminal: bool) -> usize {
        let len = self.nodes.len();

        self.nodes.push(Node {
            children: [0; 5],
            terminal,
        });

        len
    }

    fn child_idx(&self, idx: usize, chr: u8) -> usize {
        self.nodes[idx].children[char_to_idx(chr)]
    }

    pub fn insert<T: AsRef<[u8]> + ?Sized>(&mut self, key: &T) {
        let key = key.as_ref();

        let mut cur_idx = 0;

        for i in 0..key.len() {
            if self.child_idx(cur_idx, key[i]) == 0 {
                let child = self.push_child(false);

                self.nodes[cur_idx].children[char_to_idx(key[i])] = child;
            }

            cur_idx = self.child_idx(cur_idx, key[i])
        }

        self.nodes[cur_idx].terminal = true;
    }

    pub fn count<T: AsRef<[u8]> + ?Sized>(&self, key: &T) -> u64 {
        let key = key.as_ref();
        let mut cache = vec![0u64; key.len() + 1];
        cache[0] = 1;

        for i in 0..key.len() {
            if cache[i] == 0 {
                continue;
            }

            let mut cur_idx = 0;
            let mut j = i;

            while j <= key.len() {
                if self.nodes[cur_idx].terminal {
                    cache[j] += cache[i];
                }

                if j == key.len() {
                    break;
                }

                let next_idx = self.child_idx(cur_idx, key[j]);

                if next_idx == 0 {
                    break;
                }

                cur_idx = next_idx;
                j += 1;
            }
        }

        cache[key.len()]
    }
}

fn main() {
    let inputs = shared::parse_input(|s| {
        let mut lines = s.lines();

        let possible = lines.next().unwrap();
        let possible = possible.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let mut trie = Trie::new();
        for possible in possible {
            trie.insert(possible);
        }

        let _ = lines.next().unwrap();

        let towels = lines.map(ToString::to_string).collect::<Vec<_>>();

        Input {
            possible: trie,
            towels,
        }
    });

    shared::solution_fn(1, &inputs, 6, |input| {
        input
            .towels
            .iter()
            .map(|t| input.possible.count(t) > 0)
            .map(|r| r as usize)
            .sum()
    });

    shared::solution_fn(2, &inputs, 16, |input| {
        input.towels.iter().map(|t| input.possible.count(t)).sum()
    });
}

shared::runner!();
