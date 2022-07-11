mod tools;
mod methods;
mod models;

fn main() {
    //HOW to use :
    //import the required classes from the model you want to solve in the method you want to use (change the very first line of NMCS, NRPA...)
    //change BFS and NMCS models together
    //uncomment one of these lines
    
    //let st = methods::NRPA::launch_nrpa(3); //conj1
    //let st = methods::BFS::BFS(0.0, -1); //conj2
    //let st = methods::NMCS::launch_nmcs(2,  0.0, fals); // conj3
    let st = methods::NMCS::launch_nmcs(2,  0.0, false); // conj4
    //let st = methods::BFS::BFS(0.0, 0); //conj5
}
