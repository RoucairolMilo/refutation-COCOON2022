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


        //dans le cas des arbres, ces mouvmeents sont interdits
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

        //seuls mouvement autorisés en cas de construction d'arbre

        for i in 0..self.n_sommet {
            let m1 = Move{ from : i, to : self.n_sommet};
            vec.push(m1);
        }
        return vec;
    }

    pub fn charac_poly_coeffs(& self, eig : SymmetricEigen<f64, Dynamic>) -> Vec<f64>{

        let mut coeffs : Vec<f64> = Vec::new();


        for k  in 0..eig.eigenvalues.len() {
            //println!("{:?}", coeffs);
            coeffs.push(0.0);
            for a  in (1..coeffs.len()).rev() {
                coeffs[a] = coeffs[a] + coeffs[a-1]*(-eig.eigenvalues[k]);

            }
            coeffs[0] -= eig.eigenvalues[k];
        }
        coeffs.reverse();
        coeffs.push(1.0); //pour le x au bout
        return coeffs;
    }

    pub fn score(& self) -> f64{


        let DM = self.dist_matrix(); //ok pour matrices de taille 1000 (1 à 3mn)

        //println!(" matrice d'adjaccence : {}", self.adj_mat);
        println!("calcul eigen");
        let eig = SymmetricEigen::new(DM); //q
        println!("fin calcul eigen");
        //println!("eigenvalues:{}", eig.eigenvalues);
        println!("calcul charac poly coeffs");
        let delta : Vec<f64> = self.charac_poly_coeffs(eig);
        println!("fin calcul charac poly coeffs");
        //delta.sort_by(|a, b| a.partial_cmp(b).unwrap());
        //println!("eigenvalues triées:{:?}", delta);
        //println!("coefficients du polynome caractéristique :{:?}", delta);

        let mut dk : Vec<f64> = Vec::new();
        for k  in 0..self.n_sommet-1 {
            dk.push(2.0_f64.powi(k as i32)/2.0_f64.powi((self.n_sommet as i32) -2) * delta[k].abs()); //abs possible ici mais aussi plus haut
        }
        //println!("d(k):{:?}", Dk);

        // conjecture 2.7 : int(n_xommets/2) <=
        // index du pic des coefficients normalisés du polynome caractéristique de la matrice de distance du graphe
        // <= int(n*(1-1/sqrt-5)) + 1

        //trouvons l'index du pic
        let mut pdt : usize = 0;
        let mut max = 0.0;
        for i in 0..dk.len() {
            if dk[i] > max {
                max = dk[i];
                pdt = i;
            }
        }
        //(self.n_sommet as f64)*100.0-(self.n_arete as f64);
        let seuilBas :f64 = (self.n_sommet as f64/2.0_f64).floor();
        let seuilHaut : f64 = (self.n_sommet as f64 * (1.0 - 1.0/5.0_f64.sqrt())).floor()+1.0_f64;
        let sc : f64 = pdt as f64;


        //println!("--------------------------------------------------------");
        println!("sc : {},   {} <= {} <= {}", sc, seuilBas, pdt, seuilHaut);
        //println!(" matrice d'adjaccence : {}", self.adj_mat);
        //println!(" matrice de distance : {}", self.dist_matrix());
        //println!("--------------------------------------------------------");

        if sc < seuilBas || sc > seuilHaut {
            println!("VICTOIRE");
            graphToDot::adj_matrix_to_dot(self.adj_mat.clone(), "conjecture2p7");
            saveMatrix::save_matrix("adj2p7", self.adj_mat.clone());
            saveMatrix::save_matrix("dist2p7", self.dist_matrix());
        }

        //return seuilHaut - sc ;
        //return seuilBas - sc ;
        //return self.n_sommet as f64 - sc ;
        return sc ;
    }

    pub fn heuristic(&self, m : Move) -> f64{
        return 0.0;
    }

    pub fn terminal(& self) -> bool{
        return self.n_sommet>600;
    } //marche avec 800->16! (et 700->69 aussi, et 650->91, 600 -> 124, marche pas pour 550 -> 287)

    fn dist_matrix(& self) -> DMatrix<f64>{
        //on utilise une propriété sur les matrices d'adjacence, si la matrice d'adjaccence à la puissance n ne donne pas 0 dans une case, alors il y a un chein de longueur n dans la case
        let mut DM : DMatrix<f64> = self.adj_mat.clone();
        let mut An : DMatrix<f64> = self.adj_mat.clone();
        let mut tofill : usize = self.n_sommet*self.n_sommet - self.n_sommet - 2*(self.n_arete); //on retire la diagonale déjà remplie et les sommets déjà liés

        for n in 2..self.n_sommet {
            An = An*&self.adj_mat;

            //println!(" matrice de distance à n = {}  {}", n, An);

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
        println!("PAS BIEN");
        return DM; //ne devrait jamais arriver en théorie
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Move{
    pub from : usize,
    pub to : usize
}
