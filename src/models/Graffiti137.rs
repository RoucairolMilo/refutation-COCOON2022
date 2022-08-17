extern crate nalgebra;

use nalgebra::{DMatrix, Dynamic, max};
use nalgebra::linalg::SymmetricEigen;
use crate::tools::graphToDot;
use crate::tools::saveMatrix;
use crate::tools::calc;


#[derive(Clone)]
pub struct State{
    pub adj_mat : DMatrix<f64>,
    pub n_arete: usize,
    pub n_sommet: usize,
    pub seq : Vec<Move>
}

impl State{
    pub fn new() -> Self {
        Self{ adj_mat: DMatrix::from_diagonal_element(1, 1, 0.0), n_arete: 0, n_sommet: 1, seq : Vec::new() }
    }

    pub fn add_arete(&mut self, from : usize, to : i32){
        if from as i32 != to && self.n_sommet > from  {
            let mut true_to : usize = 0;
            if  to >= self.n_sommet as i32 || to == -1 {
                true_to = self.n_sommet;
                self.n_sommet +=1;
                self.adj_mat.resize_mut(self.n_sommet, self.n_sommet, 0.0)
            }else{
                true_to = to as usize;
                if self.adj_mat[(from, true_to)] != 0.0 {
                    return;
                }
            }
            self.n_arete += 1;
            self.adj_mat[(from, true_to)] = 1.0;
            self.adj_mat[(true_to, from)] = 1.0;
        }
    }

    pub fn play(&mut self, m : Move){

        self.add_arete(m.from, m.to);
        self.seq.push(m);
    }

    pub fn legal_moves(& self) ->Vec<Move>{

        let mut vec :Vec<Move> = Vec::new();

        if true {
            for i in 0..self.n_sommet {
                for j in i..self.n_sommet {
                    if self.adj_mat[(i, j)] == 0.0 {
                        let m1 = Move{ind : self.n_sommet as i32, from : i, to : j as i32};
                        vec.push(m1);
                    }
                }
            }
        }

        //tree moves
        for i in 0..self.n_sommet {
            let m1 = Move{
                ind : self.n_sommet as i32,
                from : i,
                to : -1 }; //Move{ from : i, to : self.n_sommet};
            //let m1 = Move{ from : i, to : self.n_sommet as i32 };
            vec.push(m1);
        }

        return vec;
    }


    pub fn harmonic(&self) -> f64{
        let mut sum = 0.0;
        let mut degrees = vec![];
        for i in 0..self.adj_mat.column(0).len(){
            degrees.push(self.adj_mat.column(i).sum() as f64);
        }

        for i in 0..self.adj_mat.column(0).len(){
            for j in 0..self.adj_mat.column(0).len(){
                if self.adj_mat[(i, j)] == 1.0{
                    sum += 1.0/(degrees[i] + degrees[j]);
                }
            }
        }
        return sum/2.0;
    }


    pub fn score(& self) -> f64{
        if self.n_sommet <= 5 {
            return 0.0;
        }
        let eig = SymmetricEigen::new(self.adj_mat.clone());
        //println!("eigenvalues:{}", eig.eigenvalues);

        let mut spectre : Vec<f64> = Vec::new();
        for k  in 0..eig.eigenvalues.len() {
            spectre.push(eig.eigenvalues[k]);
        }
        spectre.sort_by(|b, a| a.partial_cmp(b).unwrap());
        //println!("eigenvalues triées:{:?}", spectre);

        let sc = spectre[1] - self.harmonic();
        if sc > 0.00001 {
            println!("SOLVED");
            println!("--------------------------------------------------------");
            println!("sc : {} ", sc);
            println!("{} > {}", spectre[1], self.harmonic());
            println!(" adjacency matrix : {}", self.adj_mat);
            println!("--------------------------------------------------------");
            graphToDot::adj_matrix_to_dot(self.adj_mat.clone(), "graffiti137");
            saveMatrix::save_matrix("adjGraffiti137", self.adj_mat.clone());
        }
        return 100.0 + sc;
    }

    pub fn smoothedScore(&self) ->f64{return self.score();}

    pub fn heuristic(&self, m : Move) -> f64{ return 0.0; }

    pub fn terminal(& self) -> bool{ return self.n_sommet>6; }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub ind : i32,
    pub from : usize,
    pub to : i32
}
