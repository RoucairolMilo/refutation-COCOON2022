use nalgebra::{DMatrix, Dynamic};

pub fn softmaxChoice(l : Vec<f64>) -> usize{
    let r = rand::random::<f64>();
    let mut sum = 0.0;
    for i in 0..l.len() {
        sum += l[i].exp();
    }
    let mut sum2 = 0.0;
    for i in 0..l.len() {
        sum2+= l[i].exp()/sum;
        if sum2 >= r{
            return i;
        }
    }
    return l.len();
}

pub fn multiChoice(l : Vec<f64>, n : usize) -> Vec<usize>{
    let mut v = Vec::new();
    let mut L : Vec<f64> = l.clone();
    for i in 0..n{
        v.push(softmaxChoice(L.clone()));
        let coeff = v[v.len()-1] as f64;
        L.remove(v[v.len()-1]);
        for a in 0..L.len(){
            L[a] *= 1.0/(1.0-coeff);
        }
    }
    return v;
}

pub fn dist_matrix(adj_mat : DMatrix<f64>) -> DMatrix<f64>{
    let mut DM : DMatrix<f64> = adj_mat.clone();
    let mut An : DMatrix<f64> = adj_mat.clone();
    let n_sommet = adj_mat.column(0).len();
    let mut tofill : usize = n_sommet*n_sommet - n_sommet - (adj_mat.sum() as usize); //on retire la diagonale déjà remplie et les sommets déjà liés
    for n in 2..n_sommet {
        An = An*&adj_mat;

        for i in 0..n_sommet {
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
