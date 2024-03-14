extern crate postflop_solver;
use postflop_solver::*;
use std::mem::MaybeUninit;

struct RpsGame {
    root: MutexLike<RpsNode>,
    strategy: Vec<f32>,
    is_solved: bool,
    initial_weight: Vec<f32>,
}

struct RpsNode {
    player: usize,
    amount: i32,
}

enum Action {
    None,
}

const NUM_PRIVATE_HANDS: usize = 3;

fn find_payoff(my_card: usize, opp_card: usize) -> f32 {
    // 0 = rock, 1 = paper, 2 = scissors
    let ans = match (my_card, opp_card) {
        (0, 1) => -1.0,
        (0, 2) => 1.0,
        (1, 0) => 1.0,
        (1, 2) => -1.0,
        (2, 0) => -1.0,
        (2, 1) => 1.0,
        _ => 0.0,
    };
    ans
}

impl Game for RpsGame {
    type Node = RpsNode;

    #[inline]
    fn root(&self) -> MutexGuardLike<Self::Node> {
        self.root.lock()
    }

    #[inline]
    fn num_private_hands(&self, _player: usize) -> usize {
        NUM_PRIVATE_HANDS
    }

    #[inline]
    fn initial_weights(&self, player: usize) -> &[f32] {
        &self.inital_weight
    }

    #[inline]
    fn evaluate(
        &self,
        result: &mut [MaybeUninit<f32>],
        node: &Self::Node,
        player: usize,
        cfreach: &[f32],
    ) {
        result.iter_mut().for_each(|x| {
            x.write(0.0);
        });
        let result = unsafe { &mut *(result as *mut _ as *mut [f32]) };

        let num_hands = NUM_PRIVATE_HANDS * NUM_PRIVATE_HANDS;
        let num_hands_inv = 1.0 / num_hands as f32;
        let amount_normalized = node.amount as f32 * num_hands_inv;

        for my_card in 0..NUM_PRIVATE_HANDS {
            for opp_card in 0..NUM_PRIVATE_HANDS {
                let payoff = find_payoff(my_card, opp_card);
                // let payoff_normalized = amount_normalized * sign;
                // result[my_card] += payoff_normalized * cfreach[opp_card];
            }
        }
    }

    #[inline]
    fn chance_factor(&self, _node: &Self::Node) -> usize {
        unreachable!()
    }

    #[inline]
    fn is_solved(&self) -> bool {
        self.is_solved
    }

    #[inline]
    fn set_solved(&mut self) {
        self.is_solved = true;
    }
}
