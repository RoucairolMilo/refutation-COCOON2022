extern crate nalgebra;

use nalgebra::{DMatrix, Dynamic, max};
use nalgebra::linalg::SymmetricEigen;
use crate::tools::graphToDot;
use crate::tools::saveMatrix;


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
        /*
        for i in 0..self.n_sommet {
            for j in i..self.n_sommet {
                if self.adj_mat[(i, j)] == 0.0 {
                    let m1 = Move{ from : i, to : j};
                    let m2 = Move{ from : j, to : i};
                    vec.push(m1);
                    vec.push(m2);
                }
            }
        }

        for i in 0..self.n_sommet {
            let m1 = Move{ from : i, to : self.n_sommet};
            let m2 = Move{ from : self.n_sommet, to : i};
            vec.push(m1);
            vec.push(m2);
        }
        */
        //mode arbre uniquement
        for i in 0..self.n_sommet {
            let m1 = Move{ /*ind : self.n_sommet as i32,*/ from : i, to : -1 }; //Move{ from : i, to : self.n_sommet};
            //let m1 = Move{ from : i, to : self.n_sommet as i32 };
            vec.push(m1);
        }

        return vec;
    }

    pub fn biggest_match(& self, mut adj_mat: DMatrix<f64>) -> usize {
        // à optimiser avec du branch and bound !
        // là ça ne marche qu'avec de très petits graphes (mais ça suffit à casser cette conjecture)
        if adj_mat.max() == 0.0 {
            return 0;
        }
        //prendre un vertex ou  ne le prend pas
        let mut prend_copie = adj_mat.clone();
        'tobreak: for i in 0..self.n_sommet {
            for j in i..self.n_sommet {

                if adj_mat[(i, j)] == 1.0 {
                    adj_mat[(i, j)] = 0.0;
                    adj_mat[(j, i)] = 0.0;
                    for k in 0..self.n_sommet{
                        prend_copie[(k, j)] = 0.0;
                        prend_copie[(i, k)] = 0.0;
                        prend_copie[(j, k)] = 0.0;
                        prend_copie[(k, i)] = 0.0;
                    }
                    break 'tobreak
                }
            }
        }

        let b = 1+self.biggest_match(prend_copie);
        if b*2 >= adj_mat.column(0).len()-1 {
            return b;
        }
        let a = self.biggest_match(adj_mat);
        return max(a, b);
    }

    pub fn score(& self) -> f64{
        if self.n_sommet <= 3 {
            return 0.0;
        }

        //println!(" matrice d'adjaccence : {}", self.adj_mat);

        let eig = SymmetricEigen::new(self.adj_mat.clone());
        //println!("eigenvalues:{}", eig.eigenvalues);

        let mut spectre : Vec<f64> = Vec::new();
        for k  in 0..eig.eigenvalues.len() {
            spectre.push(eig.eigenvalues[k]);
        }
        spectre.sort_by(|a, b| a.partial_cmp(b).unwrap());
        //println!("eigenvalues triées:{:?}", spectre);
        //bloque ici, biggest match trop lourd en taille 30

        let bm =self.biggest_match(self.adj_mat.clone()) as f64;
        let sc : f64 = spectre[spectre.len()-1] + bm;
        //println!("sc : {} >= {} , biggest match : {}", sc, ((self.n_sommet-1) as f64).sqrt() + 1.0, bm);

        //println!("biggest match : {}  ", self.biggest_match(self.adj_mat.clone()) as f64);
        if sc < ((self.n_sommet-1) as f64).sqrt() + 1.0 {
            println!("VICTOIRE");
            println!("--------------------------------------------------------");
            println!("sc : {} >= {}", sc, ((self.n_sommet-1) as f64).sqrt() + 1.0);
            println!(" matrice d'adjaccence : {}", self.adj_mat);
            println!("--------------------------------------------------------");
            graphToDot::adj_matrix_to_dot(self.adj_mat.clone(), "conjecture2p1");
            saveMatrix::save_matrix("adj2p1", self.adj_mat.clone());
        }

        return (self.n_sommet*2) as f64 - sc;
    }

    pub fn smoothedScore(&self) ->f64{return self.score();}

    pub fn heuristic(&self, m : Move) -> f64{
        return 0.0;
    }

    pub fn terminal(& self) -> bool{
        return self.n_sommet>17;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    //pub ind : i32,
    pub from : usize,
    pub to : i32
}
