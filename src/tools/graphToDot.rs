use nalgebra::{DMatrix, Dynamic};
use std::io::Write;
use std::fs::{File, read};
use std::borrow::Cow;
use std::{fs, option};
use dot::{Arrow, Kind, Labeller, LabelText, Style};

use std::io::{self, BufRead};
use std::path::Path;

type Nd = isize;
type Ed = (isize,isize);
struct Edges(Vec<Ed>);

pub fn render_to<W: Write>(output: &mut W, adj_mat : DMatrix<f64>) {
    let mut ed = Vec::new();
    let n_sommets = adj_mat.row(0).len();
    for i in 0..n_sommets {
        for j in i..n_sommets {
            if adj_mat[(i, j)] == 1.0 {ed.push((i as isize, j as isize));}
        }
    }
    println!(" edges : {:?}", ed);
    let edges = Edges(ed);
    dot::render(&edges, output).unwrap();
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Edges {
    fn kind(&self) -> Kind {
        Kind::Graph
    }
    
    fn graph_id(&'a self) -> dot::Id<'a> { dot::Id::new("example1").unwrap() }

    fn node_id(&'a self, n: &Nd) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", *n)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(std::borrow::Cow::Borrowed(""))
    }

    fn edge_end_arrow(&'a self, _e: &Ed) -> Arrow {
        Arrow::none()
    }

    fn node_color(&'a self, _node: &Nd) -> Option<LabelText<'a>> {
        option::Option::Some(dot::LabelText::LabelStr("deepskyblue3".into()))
    }

    fn node_shape(&'a self, _node: &Nd) -> Option<LabelText<'a>> {
        option::Option::Some(dot::LabelText::LabelStr("circle".into()))
    }

    fn node_style(&'a self, _n: &Nd) -> Style {
        Style::Filled
    }
}

impl<'a> dot::GraphWalk<'a, Nd, Ed> for Edges {
    fn nodes(&self) -> dot::Nodes<'a,Nd> {
        // (assumes that |N| \approxeq |E|)
        let &Edges(ref v) = self;
        let mut nodes = Vec::with_capacity(v.len());
        for &(s,t) in v {
            nodes.push(s); nodes.push(t);
        }
        nodes.sort();
        nodes.dedup();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a,Ed> {
        let &Edges(ref edges) = self;
        Cow::Borrowed(&edges[..])
    }

    fn source(&self, e: &Ed) -> Nd { e.0 }

    fn target(&self, e: &Ed) -> Nd { e.1 }
}

pub fn adj_matrix_to_dot(adj_mat : DMatrix<f64>, name : &str) {

    let namext = format!("{}.dot", name);
    let mut f = File::create(namext).unwrap();
    render_to(&mut f, adj_mat)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn readMat(filename :  &str) -> DMatrix<f64>{
    let mut adj_mat : DMatrix<f64> = DMatrix::zeros(0, 0);
    let mut d = 0;
    if let Ok(lines) = read_lines(format!("savedMatrix/{}.txt", filename)) {
        d = lines.count()
    }else{
        println!("error");
        return adj_mat;
    }

    if let Ok(lines) = read_lines(format!("savedMatrix/{}.txt", filename)) {
        adj_mat = DMatrix::zeros(d, d);
        let mut i = 0;
        let mut j = 0;
        for line in lines {
            for word in line.unwrap().split(", "){
                if word != ""{
                    if word == "1"{
                        adj_mat[(i, j)] = 1.0;
                    }
                    i+=1;
                }
            }
            i = 0;
            j += 1;

        }
    }
    return adj_mat;
}



pub fn adj_matrix_file_to_dot(adj_mat_filename :  &str, name : &str) {

    let adj_mat: DMatrix<f64> = readMat(adj_mat_filename);

    let namext = format!("savedDots/{}.dot", name);
    let mut f = File::create(namext).unwrap();
    render_to(&mut f, adj_mat)
}