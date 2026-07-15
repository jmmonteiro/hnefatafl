use crate::{
    board::{Board, Team},
    cons::NUM_TILES,
    game::GameState,
};
use std::collections::{HashMap, HashSet};

use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

type TranspositionTable = HashMap<[[u8; NUM_TILES]; NUM_TILES], Node>;

#[derive(Clone)]
enum TerminalState {
    Unknown,
    NotTerminal,
    Terminal(Team),
}

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
        transposition_table
            .entry(root)
            .or_insert_with(|| Node::new(HashSet::new(), root, starting_team));
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
        Node::new(
            match parent_state {
                None => HashSet::new(),
                Some(p) => HashSet::from([p]),
            },
            node_state,
            team,
        )
    });

    // -- Base cases
    match transposition_table.get(&node_state).unwrap().terminal_state {
        TerminalState::NotTerminal => {}
        TerminalState::Terminal(t) => {
            todo!("backpropagate and return")
        }
        TerminalState::Unknown => {
            let (_, game_state, _) =
                Board::new(node_state, None).get_board_after_move_piece(0, 0, 0, 0, &team);
            if let GameState::GameOver = game_state {
                transposition_table
                    .get_mut(&node_state)
                    .unwrap()
                    .terminal_state = TerminalState::Terminal(team.clone());
                todo!("backpropagate and return")
            }
        }
    }

    // -- If this node has not been visited, then rollout, backpropagate, and return
    let node = transposition_table.get(&node_state).unwrap();
    if node.n == 0.0 {
        let winning_team = node.rollout();
        todo!("backpropagate and return")
    }

    // -- Expand
    transposition_table.get_mut(&node_state).unwrap().expand();

    // -- Select a node
    fn ucb1(node: &Node, parent_n: f32) -> f32 {
        node.v + 2.0 * (parent_n.ln() / node.n).sqrt()
    }

    let max_child = transposition_table
        .get(&node_state)
        .unwrap()
        .children
        .iter()
        .max_by(|a, b| {
            let node_n = transposition_table.get(&node_state).unwrap().n;
            let a_score = ucb1(transposition_table.get(*a).unwrap(), node_n);
            let b_score = ucb1(transposition_table.get(*b).unwrap(), node_n);
            a_score.partial_cmp(&b_score).unwrap()
        })
        .copied()
        .unwrap();

    // -- Call function recursively, keep going deeper
    walk_tree(
        transposition_table.get(&max_child).unwrap().state,
        parent_state,
        match team {
            Team::Attacker => Team::Defender,
            Team::Defender => Team::Attacker,
        },
        transposition_table,
    );
}

#[derive(Clone)]
struct Node {
    n: f32,
    v: f32,
    children: HashSet<[[u8; NUM_TILES]; NUM_TILES]>,
    parents: HashSet<[[u8; NUM_TILES]; NUM_TILES]>,
    terminal_state: TerminalState,
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
            terminal_state: TerminalState::Unknown,
            team,
            state,
        }
    }
    fn expand(&mut self) {
        // Originally, you would not expand all nodes. But I don't want to perform this computation
        // over and over again
        if self.children.is_empty() {
            self.children.extend(
                Board::new(self.state, None)
                    .get_possible_moves(&self.team)
                    .iter()
                    .map(|m| m.get_state_as_int()),
            );
        }
    }

    fn rollout(&self) -> Team {
        if let TerminalState::Terminal(t) = self.terminal_state {
            return t;
        }
        // Randomly play out the board, until you reach a result

        let mut node = self.clone();
        let mut rng = rand::rng();
        while matches!(node.terminal_state, TerminalState::NotTerminal) {
            node.expand();
            node = Node::new(
                [node.state].into(),
                *node.children.iter().choose(&mut rng).unwrap(),
                match node.team {
                    Team::Attacker => Team::Defender,
                    Team::Defender => Team::Attacker,
                },
            );
        }
        match node.terminal_state {
            TerminalState::NotTerminal | TerminalState::Unknown => {
                panic!("Terminal state not reached with a winner. This should never happen")
            }
            TerminalState::Terminal(t) => t,
        }
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
}
