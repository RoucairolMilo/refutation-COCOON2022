extern crate nalgebra;
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

    pub fn add_arete(&mut self, from : usize, to : usize){
        if from != to && self.n_sommet > from  {
            let mut true_to : usize = to;
            if  to >= self.n_sommet {
                true_to = self.n_sommet;
                self.n_sommet +=1;
                self.adj_mat.resize_mut(self.n_sommet, self.n_sommet, 0.0)
            }else{
                if self.adj_mat[(from, to)] != 0.0 {
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

        //only moves allowed for tree buildings
        for i in 0..self.n_sommet {
            let m1 = Move{ from : i, to : self.n_sommet};
            vec.push(m1);
        }
        return vec;
    }

    pub fn charac_poly_coeffs(& self, eig : SymmetricEigen<f64, Dynamic>) -> Vec<f64>{

        let mut coeffs : Vec<f64> = Vec::new();


        for k  in 0..eig.eigenvalues.len() {
            coeffs.push(0.0);
            for a  in (1..coeffs.len()).rev() {
                coeffs[a] = coeffs[a] + coeffs[a-1]*(-eig.eigenvalues[k]);

            }
            coeffs[0] -= eig.eigenvalues[k];
        }
        coeffs.reverse();
        coeffs.push(1.0);
        return coeffs;
    }

    pub fn score(& self) -> f64{

        if self.n_sommet <= 1 {
            return 0.0;
        }


        let DM = self.dist_matrix();

        let eig = SymmetricEigen::new(DM);
        //println!("eigenvalues:{}", eig.eigenvalues);

        let mut delta : Vec<f64> = self.charac_poly_coeffs(eig);

        let mut dk : Vec<f64> = Vec::new();
        for k  in 0..self.n_sommet-1 {
            dk.push(2.0_f64.powi(k as i32)/2.0_f64.powi((self.n_sommet as i32) -2) * delta[k].abs()); //abs possible ici mais aussi plus haut
        }

        //let's find the peak index
        let mut pdt : usize = 0;
        let mut max = 0.0;
        for i in 0..dk.len() {
            if dk[i] > max {
                max = dk[i];
                pdt = i;
            }
        }

        let eigAdj = SymmetricEigen::new(self.adj_mat.clone());
        //println!("eigenvalues:{}", eig.eigenvalues);

        let coeffAdj : Vec<f64> = self.charac_poly_coeffs(eigAdj);

        //extract non zero
        let mut coeffAdjNonZero : Vec<f64> = Vec::new();
        for i in 0..coeffAdj.len() {
            if coeffAdj[i] != 0.0 {
                coeffAdjNonZero.push(coeffAdj[i]);
            }
        }


        let mut pat : usize = 0;
        max = 0.0;
        for i in 0..coeffAdjNonZero.len() {
            if coeffAdjNonZero[i] > max {
                max = coeffAdjNonZero[i];
                pat = i;
            }
        }

        let sc : f64 = (pat as f64)/(coeffAdjNonZero.len() as f64) - (1.0 - (pdt as f64)/(dk.len() as f64));

        if sc.abs() > 0.1 {
            println!("PAT : {}/{}   PDT : {}/{}", pat, coeffAdjNonZero.len(), pdt, dk.len());
            saveMatrix::save_matrix("adj3", self.adj_mat.clone());
            saveMatrix::save_matrix("dist3", self.dist_matrix());
        }
        return sc.abs() ;
    }

    pub fn smoothedScore(&self)->f64{
        return 0.0;
    }

    pub fn heuristic(&self, m : Move) -> f64{
        return 0.0;
    }

    pub fn terminal(& self) -> bool{
        return self.n_sommet>30;
    }

    fn dist_matrix(& self) -> DMatrix<f64>{
        let mut DM : DMatrix<f64> = self.adj_mat.clone();
        let mut An : DMatrix<f64> = self.adj_mat.clone();
        let mut tofill : usize = self.n_sommet*self.n_sommet - self.n_sommet - 2*(self.n_arete); //on retire la diagonale déjà remplie

        for n in 2..self.n_sommet {
            An = An*&self.adj_mat;

            for i in 0..self.n_sommet {
                for j in 0..i {
                    if An[(i, j)] != 0.0 && DM[(i, j)] == 0.0 {
                        DM[(i, j)] = n as f64;
                        DM[(j, i)] = n as f64;
                        tofill -= 2;
                        if tofill == 0 {return DM;}
                    }
                }
            }
        }
        return DM;
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub from : usize,
    pub to : usize
}
