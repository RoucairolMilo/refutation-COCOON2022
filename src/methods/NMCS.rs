use crate::models::conjecture2::{State};
use crate::tools::calc::softmaxChoice;

pub struct NMCS{
    pub best_yet : f64
}

impl NMCS{
    pub fn new() -> Self {
        Self{ best_yet: 0.0 }
    }

    pub fn playout(&mut self, mut st: State, heuristic_w : f64) -> State{
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

    pub fn nmcs(&mut self, mut st: State, n : i8, heuristic_w : f64, verbose : bool) -> State{
        let mut best_state: State = st.clone(); //State::new();
        let mut best_state_score = -1.0; //best_state.score();
        while !st.terminal(){
            let moves = st.legal_moves();
            if moves.len() == 0 {return st;}
            for &mv in &moves{
                let mut new_st = st.clone();
                new_st.play(mv);
                if n <= 1 {
                    new_st = self.playout(new_st, heuristic_w);
                }else{
                    new_st = self.nmcs(new_st, n-1, heuristic_w, verbose);
                }
                let new_st_score = new_st.score();
                if new_st_score > best_state_score {
                    best_state = new_st;
                    best_state_score = new_st_score;
                    if best_state_score > self.best_yet {
                        self.best_yet = best_state_score;
                        if verbose {println!("best score yet : {}", best_state_score);}
                    }
                }
            }
            st.play(best_state.seq[st.seq.len()]);
        }
        return st;
    }
}

pub fn launch_nmcs(level : i8, heuristic_w : f64, verbose : bool) -> State{
    let mut expe = NMCS::new();
    let mut st = State::new();
    st = expe.nmcs(st, level, heuristic_w, verbose);
    return st;
}
