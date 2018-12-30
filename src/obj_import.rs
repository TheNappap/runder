
use std::fs::File;
use std::io::{Error, BufReader, BufRead};

use primitives::{Triangle, TriangleMesh};
use math::{Point, Transformation};
use material::{ Color, Lambertian};

pub fn parse_obj(file_path: &str) -> Result<TriangleMesh,Error> {
    let file = File::open(file_path)?;
    let mut vertices: Vec<Vec<String>> = vec![];
    let mut faces: Vec<Vec<String>> = vec![];

    for line_result in BufReader::new(file).lines() {
        let line = line_result?;
        match line.chars().next() {
            Some('v') => vertices.push(line.split(char::is_whitespace).skip(1).map(|s: &str| s.to_string()).collect() ),
            Some('f') => faces.push(line.split(char::is_whitespace).skip(1).map(|s: &str| s.to_string()).collect() ),
            _ => ()
        }
    }

    let vertices : Vec<Point> = vertices.iter().map(|vec| {
        let v: Vec<f64> = vec.iter().map(|s| s.parse().expect("Unable to convert String to f64") ).collect();
        Point::new(v[0], v[1], v[2])
    }).collect();

    let transformation = Transformation::new();
    let material = Box::new(Lambertian::new(Color::gray(1.0)));

    let trans = transformation.clone();
    let mat = material.clone();
    let faces : Vec<Triangle> = faces.iter().map(move |vec|{
        let v: Vec<Point> = vec.iter().map(|s| s.parse().expect("Unable to convert String to usize") )
            .map(|index: usize| vertices.get(index-1).unwrap() ).cloned().collect();
        Triangle::new([v[0],v[1],v[2]], trans.clone(), mat.clone())
    }).collect();

    let mesh = TriangleMesh::new(faces, transformation, material);
    Ok( mesh )
}