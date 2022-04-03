use crate::models::conjecture1::{State, Move};
use std::collections::HashMap;
use std::ptr::hash;

const T : f64 = 1.0;
const alpha : f64 = 0.1;

pub fn random_move(st : &State, moves : Vec<Move>, policy: &mut HashMap<Move, f64>, heuristic_w: f64) -> Move{
    let mut sum : f64 = 0.0;

    let mut hash_heuri: HashMap<Move, f64> = HashMap::new();

    for &mv  in &moves {

        hash_heuri.insert(mv, st.heuristic(mv)* heuristic_w);

        match policy.get(&mv){
            Some(v) => sum += (v/T + hash_heuri.get(&mv).unwrap()).exp(),
            None => {
                policy.insert(mv, *hash_heuri.get(&mv).unwrap());
                sum+= hash_heuri.get(&mv).unwrap().exp()/T
            }
        };
    }
    let stop = sum*rand::random::<f64>();
    sum = 0.0;
    for &mv in &moves {
        sum += (policy.get(&mv).unwrap()/T + hash_heuri.get(&mv).unwrap()).exp() ;
        if sum > stop {
            return mv;
        }
    }
    return moves[0];
}

pub fn playout(mut st : State, mut policy : HashMap<Move, f64>, heuristic_w: f64) -> State{

    while !st.terminal() {
        let  moves: Vec<Move> = st.legal_moves();
        if moves.len() == 0 {
            return st;
        }
        let mv = random_move(&st, moves, &mut policy, heuristic_w);
        st.play(mv);
    }
    return st;
}

pub fn adapt(mut policy: HashMap<Move, f64>, st : &mut State, heuristic_w : f64) -> HashMap<Move, f64>{
    let mut s = State::new();
    let mut polp: HashMap<Move, f64> = policy.clone();

    for best in & st.seq[..] {
        let moves = s.legal_moves();
        let mut z = 0.0;
        let mut hash_heuri: HashMap<Move, f64> = HashMap::new();
        for &m in &moves {
            hash_heuri.insert(m, s.heuristic(m)* heuristic_w);
            match policy.get(&m){
                Some(v) => z += (v/T + hash_heuri.get(&m).unwrap()).exp(),
                None => {policy.insert(m, hash_heuri.get(&m).unwrap().exp()); z += policy.get(&m).unwrap()}
            };
        }

        for &m in &moves {
            let mut delta = 0.0;
            if &m == best {delta = 1.0}
            match polp.get(&m){
                Some(v) => polp.insert(m, v - alpha/T *  (policy.get(&m).unwrap().exp()/z - delta)),
                None => polp.insert(m, -policy.get(&m).unwrap().exp()/z)
            };
        }

        s.play(*best);
    }
    return polp;
}

pub fn gnrpa(level : i8, mut policy: HashMap<Move, f64>, heuristic_w: f64, initial : bool) -> State {
    let mut st: State = State::new();
    let mut stscore : f64 = st.score();

    if level == 0 {
        return playout(st, policy, heuristic_w);
    }

    for i in 0..100 {
        if initial {println!("GNRPA loop {}, best score : {} ", i, stscore);}
        let pol : HashMap<Move, f64> = policy.clone();
        let mut s = gnrpa(level-1, pol, heuristic_w, false);

        let s_score = s.score();

        if stscore < s_score {
            st = s;
            stscore = s_score;
        }
        policy = adapt(policy, &mut st, heuristic_w);

        if level == 1 {
            //println!("best yet : {}, {}, {}",stscore, st.n_sommet, st.n_arete );
        }
    }

    return st;
}

pub fn launch_gnrpa(level : i8, heuristic_w: f64) -> State {
    let mut policy = HashMap::new();
    let st = gnrpa(level, policy, heuristic_w, true);
    return st;
}
