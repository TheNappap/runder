
use std::fs::File;
use std::io::{Error, BufReader, BufRead};
use std::f64;

use super::{Rectangle, Triangle, Mesh, Face, Lambertian};
use crate::math::{Point};
use crate::cg_tools::{Color};

pub fn parse_obj(file_path: &str) -> Result<Mesh,Error> {
    let file = File::open(file_path)?;
    let mut vertices: Vec<Vec<String>> = vec![];
    let mut faces: Vec<Vec<String>> = vec![];

    for line_result in BufReader::new(file).lines() {
        let line = line_result?;
        let mut chars = line.chars();
        match chars.next() {
            Some('v') => match chars.next() {
                Some(' ') => vertices.push(line.split(char::is_whitespace).skip(1).map(|s: &str| s.to_string()).collect() ),
                _ => ()
            }
            Some('f') => faces.push(line.split(char::is_whitespace).skip(1).map(|s: &str| s.to_string()).collect() ),
            _ => ()
        }
    }

    let vertices : Vec<Point> = vertices.iter().map(|vec| {
        let v: Vec<f64> = vec.iter().map(|s| s.parse().expect("Unable to convert String to f64") ).collect();
        Point::new(v[0], v[1], v[2])
    }).collect();

    let material = Box::new(Lambertian::new(Color::gray_scale(1.0)));
    let mat = material.clone();
    let faces : Vec<Box<dyn Face>> = faces.iter().filter_map(move |vec|{
        let v: Vec<Point> = vec.iter().map(|s| s.split('/').next().unwrap() )
            .map(|s| s.parse().expect("Unable to convert String to usize") )
            .map(|index: usize| vertices.get(index-1).unwrap() ).cloned().collect();
        match v.len() {
            3 => Some(Box::new(Triangle::new([v[0],v[1],v[2]], false,mat.clone())) as Box<dyn Face>),
            4 => Some(Box::new(Rectangle::new([v[0],v[1],v[2],v[3]], false,mat.clone())) as Box<dyn Face>),
            x if x < 3 => None,
            x => {
                println!("Faces with {} vertices are not supported.", x);
                None
            }
        }
    }).collect();

    println!("Imported mesh: {}", file_path);
    println!("Amount of faces: {}", faces.len());
    let mesh = Mesh::new(faces, material);
    Ok( mesh )
}