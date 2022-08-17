extern crate nalgebra;

use std::f64::consts::PI;
use nalgebra::{DMatrix, Dynamic};
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

        self.add_arete(m.from, -1);
        self.seq.push(m);
    }

    pub fn legal_moves(& self) ->Vec<Move>{

        let mut vec :Vec<Move> = Vec::new();

        //moves for any graph
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
        }*/

        //moves for trees only
        for i in 0..self.n_sommet {
            let m1 = Move{ind : self.n_sommet, from : i}; //Move{ from : i, to : self.n_sommet};
            vec.push(m1);
        }
        return vec;
    }

    pub fn score(& self) -> f64{
        if self.n_sommet <= 4 {
            return 0.0;
        }

        let dm = self.dist_matrix();
        let dm2 = dm.clone();
        let mut d = dm.max();

        let eig = SymmetricEigen::new(dm);
        //println!("eigenvalues:{}", eig.eigenvalues);

        let mut spectre : Vec<f64> = Vec::new();
        for k  in 0..eig.eigenvalues.len() {
            spectre.push(eig.eigenvalues[k]);
        }
        spectre.sort_by(|b, a| a.partial_cmp(b).unwrap());
        //println!("eigenvalues triées:{:?}", spectre);

        let mut minimumDistanceVert = dm2.row(0).sum();
        for i in 1..self.n_sommet {
            let temp = dm2.row(i).sum();
            if  temp < minimumDistanceVert {
                minimumDistanceVert = temp;
            }
        }

        let proximity : f64 = 1.0 /(self.n_sommet as f64 -1.0) * minimumDistanceVert;


        //println!("{}", (2.0* d /3.0).floor() as usize -1);

        let sc : f64 = proximity + spectre[(2.0* d /3.0).floor() as usize -1];
        if sc <= 0.0 {
            println!("SOLVED");
            println!("--------------------------------------------------------");
            println!("sc : {}", sc);
            println!(" adjacency matrix : {}", self.adj_mat);
            println!(" distance matrix : {}", self.dist_matrix());
            println!("--------------------------------------------------------");
            graphToDot::adj_matrix_to_dot(self.adj_mat.clone(), "conjecture2");
            saveMatrix::save_matrix("adj2", self.adj_mat.clone());
            saveMatrix::save_matrix("dist2", self.dist_matrix());
        }

        return 10.0-sc;
    }

    pub fn smoothedScore(&self) -> f64{
        if self.n_sommet <= 4 {
            return 0.0;
        }

        let dm = self.dist_matrix();
        let dm2 = dm.clone();
        let mut d = dm.max();

        let eig = SymmetricEigen::new(dm);
        //println!("eigenvalues:{}", eig.eigenvalues);

        let mut spectre : Vec<f64> = Vec::new();
        for k  in 0..eig.eigenvalues.len() {
            spectre.push(eig.eigenvalues[k]);
        }
        spectre.sort_by(|b, a| a.partial_cmp(b).unwrap());

        let mut minimumDistanceVert = dm2.row(0).sum();
        for i in 1..self.n_sommet {
            let temp = dm2.row(i).sum();
            if  temp < minimumDistanceVert {
                minimumDistanceVert = temp;
            }
        }

        let proximity : f64 = 1.0 /(self.n_sommet as f64 -1.0) * minimumDistanceVert;


        //println!("{}", (2.0* d /3.0).floor() as usize -1);


        let mut interpSpectre = Vec::new();
        interpSpectre.push(spectre[0]);
        for i in 0..spectre.len()-1 {
            interpSpectre.push(spectre[i] + 1.0/3.0*(spectre[i+1]-spectre[i]));
            interpSpectre.push(spectre[i] + 2.0/3.0*(spectre[i+1]-spectre[i]));
            interpSpectre.push(spectre[i+1]);
        }

        let sc : f64 = proximity + interpSpectre[2* d as usize -3];
        if sc <= 0.0 {
            println!("SOLVED ?");
        }

        return 10.0-sc;
    }

    pub fn heuristic(&self, m : Move) -> f64{ return 0.0; }

    pub fn terminal(& self) -> bool{
        return self.n_sommet>210;
    }

    fn dist_matrix(& self) -> DMatrix<f64>{
        //on utilise une propriété sur les matrices d'adjacence, si la matrice d'adjaccence à la puissance n ne donne pas 0 dans une case, alors il y a un chein de longueur n dans la case
        let mut dm: DMatrix<f64> = self.adj_mat.clone();
        let mut an: DMatrix<f64> = self.adj_mat.clone();
        let mut tofill : usize = self.n_sommet*self.n_sommet - self.n_sommet - 2*(self.n_arete); //on retire la diagonale déjà remplie

        for n in 2..self.n_sommet {
            an = an *&self.adj_mat;

            //println!(" matrice de distance à n = {}  {}", n, An);

            for i in 0..self.n_sommet {
                for j in 0..i {
                    if an[(i, j)] != 0.0 && dm[(i, j)] == 0.0 {
                        dm[(i, j)] = n as f64;
                        dm[(j, i)] = n as f64;
                        tofill -= 2;
                        if tofill == 0 {return dm;}
                    }
                }
            }
        }
        return dm;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub ind : usize,
    pub from : usize,
}
