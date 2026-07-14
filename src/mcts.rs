use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type NodeRef = Rc<RefCell<Node>>;
type TranspositionTable = HashMap<[[u8; NUM_TILES]; NUM_TILES], NodeRef>;

use crate::{
    board::{Board, Team},
    cons::NUM_TILES,
};
struct Tree {
    root: [[u8; NUM_TILES]; NUM_TILES],
    starting_team: Team,
}
impl Tree {
    fn new(
        root: [[u8; NUM_TILES]; NUM_TILES],
        starting_team: Team,
        transposition_table: &mut TranspositionTable,
    ) -> Tree {
        transposition_table.entry(root).or_insert_with(|| {
            Rc::new(RefCell::new(Node::new(HashSet::new(), root, starting_team)))
        });
        Tree {
            root,
            starting_team,
        }
    }
}

fn walk_tree(
    node_state: [[u8; NUM_TILES]; NUM_TILES],
    parent_state: Option<[[u8; NUM_TILES]; NUM_TILES]>,
    team: Team,
    transposition_table: &mut TranspositionTable,
) {
    transposition_table.entry(node_state).or_insert_with(|| {
        Rc::new(RefCell::new(Node::new(
            match parent_state {
                None => HashSet::new(),
                Some(p) => HashSet::from([p]),
            },
            node_state,
            team,
        )))
    });

    // -- Base case: If this is a leaf node, expand and backprob
    if transposition_table
        .get(&node_state)
        .unwrap()
        .borrow_mut()
        .expand()
    {
        // Clone the Rc — cheap, just increments a reference count
        let node_rc = Rc::clone(transposition_table.get(&node_state).unwrap());
        // No borrow on the table is held here, so we can pass &mut transposition_table freely
        node_rc.borrow_mut().backpropagate(transposition_table);
        return;
    }
    // -- Select a node
    fn ucb1(node: &Node, parent_n: f32) -> f32 {
        node.v + 2.0 * (parent_n.ln() / node.n).sqrt()
    }
    let children: Vec<_> = transposition_table
        .get(&node_state)
        .unwrap()
        .borrow()
        .children
        .iter()
        .cloned()
        .collect(); // borrow released here

    let max_child = children
        .iter()
        .max_by(|a, b| {
            let node_n = transposition_table.get(&node_state).unwrap().borrow().n;
            let a_score = ucb1(
                &transposition_table.get(a.clone()).unwrap().borrow(),
                node_n,
            );
            let b_score = ucb1(
                &transposition_table.get(b.clone()).unwrap().borrow(),
                node_n,
            );
            a_score.partial_cmp(&b_score).unwrap()
        })
        .copied()
        .unwrap();

    // -- Call function recursively, keep going deeper
    walk_tree(
        transposition_table.get(&max_child).unwrap().get_mut().state,
        parent_state,
        match team {
            Team::Attacker => Team::Defender,
            Team::Defender => Team::Attacker,
        },
        transposition_table,
    );
}

struct Node {
    n: f32,
    v: f32,
    children: HashSet<[[u8; NUM_TILES]; NUM_TILES]>,
    parents: HashSet<[[u8; NUM_TILES]; NUM_TILES]>,
    is_terminal: bool,
    team: Team,
    state: [[u8; NUM_TILES]; NUM_TILES],
}

impl Node {
    fn new(
        parents: HashSet<[[u8; NUM_TILES]; NUM_TILES]>,
        state: [[u8; NUM_TILES]; NUM_TILES],
        team: Team,
    ) -> Node {
        Node {
            n: 0.,
            v: 0.,
            children: HashSet::new(),
            parents,
            is_terminal: true,
            team,
            state,
        }
    }
    fn expand(&mut self) -> bool {
        // Originally, you would not expand all nodes. But I don't want to perform this computation
        // over and over again
        if self.children.is_empty() {
            self.children.extend(
                Board::new(self.state, None)
                    .get_possible_moves(&self.team)
                    .iter()
                    .map(|m| m.get_state_as_int()),
            );
            return true;
        }
        false
    }
    fn backpropagate(&mut self, transposition_table: &mut TranspositionTable) -> f32 {
        todo!()
        // if self.is_terminal {
        //     let (is_terminal, value) = self.get_value_of_board();
        //
        //     // set terminal state
        //     transposition_table
        //         .get_mut(&self.state)
        //         .unwrap()
        //         .is_terminal = is_terminal;
        //
        //     return value;
        // }
        // value
    }

    fn get_value_of_board(&self) -> (bool, f32) {
        // TODO: Actually implement this function
        (false, 1.0) // is terminal, value
    }
}
