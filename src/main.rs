mod tools;
mod methods;
mod models;

fn main() {
    //HOW to use :
    //import the required classes from the model you want to solve in the method you want to use (change the very first line of NMCS, NRPA...)
    //change BFS and NMCS models together (they are tied)
    //uncomment one of these lines
    
    //let st = methods::NRPA::launch_nrpa(3); //conj1, may lock itself in local minimum near the conjecture treshold, relaunch a few times
    //let st = methods::BFS::BFS(0.0, -1); //conj2, wait for the score to go past 10, may take some time (5 to 10mn)
    //let st = methods::NMCS::launch_nmcs(2,  0.0, false); // conj3
    //let st = methods::BFS::BFS(0.0, 0); //conj4 (Graffiti137)
}
