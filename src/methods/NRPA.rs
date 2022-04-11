use crate::models::conjecture1::{State, Move};
use std::collections::HashMap;


pub fn random_move(moves : Vec<Move>, policy: &mut HashMap<Move, f64>) -> Move{

    let mut sum : f64 = 0.0;

    for &mv  in &moves {
        match policy.get(&mv){
            Some(v) => sum += v.exp(),
            None => {policy.insert(mv, 0.0); sum+=1.0}
        };

    }

    let stop = sum*rand::random::<f64>();
    sum = 0.0;
    for &mv in &moves {
        sum += policy.get(&mv).unwrap().exp();
        if sum > stop {
            return mv;
        }
    }
    return moves[0];
}

pub fn playout(mut st : State, mut policy : HashMap<Move, f64>) -> State{
    while !st.terminal() {
        let  moves: Vec<Move> = st.legal_moves();
        if moves.len() == 0 {
            return st;
        }
        let mv = random_move(moves, &mut policy);
        st.play(mv);
    }
    //println!("playout : {}, {}, {}",st.score(), st.n_sommet, st.n_arete );
    return st;
}

pub fn adapt(mut policy: HashMap<Move, f64>, st : &mut State) -> HashMap<Move, f64>{
    let mut s = State::new();
    let mut polp: HashMap<Move, f64> = policy.clone();
    for best in &mut st.seq[..] {
        let moves = s.legal_moves();
        let mut sum = 0.0;
        for &m in &moves {
            match policy.get(&m){
                Some(v) => sum += v.exp(),
                None => {policy.insert(m, 0.0); sum += 1.0}
            };
        }

        for &m in &moves {
            match polp.get(&m){
                Some(v) => polp.insert(m, v - policy.get(&m).unwrap().exp()/sum),
                None => polp.insert(m, -policy.get(&m).unwrap().exp()/sum)
            };

        }
        polp.insert(*best, polp.get(&best).unwrap() + 1.0);
        s.play(*best);
    }
    return polp;
}

pub fn nrpa(level : i8, mut policy: HashMap<Move, f64>, initial : bool) -> State {
    let mut st: State = State::new();
    let mut stscore : f64 = st.score();

    if level == 0 {
        return playout(st, policy);
    }

    for i in 0..100 {
        if initial {println!("NRPA loop {}, best score : {} ", i, stscore);
            //println!("{}", st.adj_mat);
        }
        let pol : HashMap<Move, f64> = policy.clone();
        let mut s = nrpa(level-1, pol, false);
        let s_score = s.score();
        if stscore < s_score {
            st = s;
            stscore = s_score;
        }
        policy = adapt(policy, &mut st);
    }
    return st;
}

pub fn launch_nrpa(level : i8) -> State {
    let mut policy = HashMap::new();
    let st = nrpa(level, policy, true);
    return st;
}
