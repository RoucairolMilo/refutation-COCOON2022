mod tests;
mod tools;
mod methods;
mod models;

fn main() {
    //HOW to use :
    //import the required classes from the model you want to solve in teh method you want to use (change the very first line of NMCS, NRPA...)
    //uncomment one of these lines
    
    //let st = methods::NRPA::launch_nrpa(3); //conj1
    //let st = methods::TS::table_sampling(0.0, -1); //conj2
    //let st = methods::NMCS::launch_nmcs(2,  0.0); // conj3
    //let st = methods::NMCS::launch_nmcs(2,  0.0); // conj4
    //let st = methods::BFS::BFS(0.0, 0); //conj5
}
