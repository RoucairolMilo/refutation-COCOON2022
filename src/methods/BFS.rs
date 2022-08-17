use nalgebra::min;
use crate::models::Graffiti137::{State, Move};
use crate::tools::calc::softmaxChoice;
use crate::methods::NMCS::launch_nmcs;

#[derive(Clone)]
pub struct WS{ //Weight-State
    pub w : f64,
    pub s : State

}

pub fn insertDicho(l : &Vec<WS>, node : &WS) -> usize{
    let mut i = l.len()/2;
    let mut mi = 0;
    let mut ma = l.len()-1;
    while (i!=0 && l[i-1].w > node.w) || l[i].w < node.w{
        if l[i].w == node.w {
            return i;
        }
        if l[i].w < node.w{
            mi = i + 1;
            if mi> ma{
                return mi;
            }
        }else{
            ma = i;
        }
        i = (mi as f64/2.0 + (ma as f64)/2.0) as usize
    }
    return i;
}

pub fn playout(mut st: State, heuristic_w : f64) -> State{
    while !st.terminal() {
        let moves = st.legal_moves();
        if moves.len() == 0 {return st;}
        let mut i = ((moves.len() as f64)*rand::random::<f64>()) as usize;
        if heuristic_w != 0.0 {
            let mut weights = Vec::new();
            for &m in &moves{
                weights.push(heuristic_w*st.heuristic(m));
            }
            i = softmaxChoice(weights);
        }
        let mv = moves[i];
        st.play(mv);
    }
    return st;
}

pub fn BFS(heuristic_w : f64, p:i32) -> State{

    let mut st = State::new();
    let mut open_nodes = Vec::new();
    open_nodes.push(WS{w : 0.0, s : st.clone()});

    let mut best_score_yet = 0.0;
    let mut best_state_yet = st.clone();

    while open_nodes.len() != 0 {
        let mut node = open_nodes.pop().unwrap();
        for m in node.s.legal_moves() {
            let mut new_state = node.s.clone();
            new_state.play(m);


            if p >= 0 {
                let mut best_playout_state = launch_nmcs(1, heuristic_w, false);//playout(new_state.clone(), heuristic_w);
                let mut best_playout_state_score = best_playout_state.score();
                for i in 0..p{
                    let playout_state = launch_nmcs(1, heuristic_w, false);//playout(new_state.clone(), heuristic_w);
                    let playout_state_score = playout_state.score();

                    if playout_state_score > best_playout_state_score {
                        best_playout_state = playout_state.clone();
                        best_playout_state_score = playout_state_score;
                    }
                }
                if best_playout_state_score > best_score_yet{
                    best_score_yet = best_playout_state_score;
                    best_state_yet = best_playout_state.clone();
                    println!("BFS best score beaten ! {}", best_score_yet);
                }

                let new_ws = WS{w : best_playout_state_score, s : new_state};
                let mut i = 0;
                if open_nodes.len() != 0 {
                    i = insertDicho(&open_nodes, &new_ws);
                }
                open_nodes.insert(i, new_ws);
            }else{
                let smoothed_score = new_state.smoothedScore(); //an estimation on wether keep going on that child
                if smoothed_score > best_score_yet{
                    let new_state_score = new_state.score();
                    if new_state_score > best_score_yet{
                        best_score_yet = new_state_score;
                        best_state_yet = new_state.clone();
                        println!("BFS best score beaten  ! {}", best_score_yet);
                        //for i in 0..open_nodes.len() {print!("{} ", open_nodes[open_nodes.len() -1 - i].w);}
                        println!(" ");
                    }
                }
                if !new_state.terminal() {
                    let new_ws = WS{w : smoothed_score, s : new_state};

                    let mut i = 0;
                    if open_nodes.len() != 0 {
                        i = insertDicho(&open_nodes, &new_ws);
                    }
                    open_nodes.insert(i, new_ws);
                }
            }
        }
    }
    return best_state_yet;
}
