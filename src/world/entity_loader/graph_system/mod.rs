use bevy::log::info;
use array2d::{Array2D, Error};

pub trait GraphBaseEvent{
    fn gen_empty_matrix(&mut self, num_vertices: usize);
    fn add_edge(&mut self, v1: usize, v2: usize, weight: u32);
    fn get_adj_vertices(&mut self, v: usize) -> Vec<u32>;
    fn get_edge_weight(&mut self, v1: usize, v2: usize) -> Option<&u32>;
    fn display(&mut self);
    fn num_of_vertices(&mut self) -> usize;
}

pub struct AdjMatrixGraph{
    pub num_vertices: usize,
    pub directed: bool,
    pub matrix: Array2D<u32>
}

impl GraphBaseEvent for AdjMatrixGraph{
    fn gen_empty_matrix(&mut self, num_vertices: usize) {
        self.num_vertices = num_vertices;
        for rows in 0..=num_vertices {
            for cols in 0..=num_vertices {
                self.matrix = Array2D::filled_with(0, rows, cols);
            }
        }
    }

    fn add_edge(&mut self, v1: usize, v2: usize, weight: u32) {
        if v1 >= self.num_vertices.try_into().unwrap() || v2 >= self.num_vertices.try_into().unwrap() {
            info!("Vertices are out of bounds");
        }

        if weight < 1 {
            info!("Weight cannot be less than 1");
        }

        // self.matrix[v1][v2] = weight;
        let _ = self.matrix.set(v1, v2, weight);

        //In an undirected graph all edges are bi-directional
        if !self.directed {
            // self.matrix[v2][v1] = weight;
            let _ = self.matrix.set(v2, v1, weight);

        }
    }

    fn get_adj_vertices(&mut self, v: usize) -> Vec<u32> {
        if v < 0 || v >= self.num_vertices.try_into().unwrap() {
            info!("Cannot access vertex");
        }

        let mut adjacent_vertices: Vec<u32> = Vec::new();
        for i in 0..=self.num_vertices {
            if self.matrix.get(v, i) > Some(&0) {
                // adjacent_vertices.Add(i);
                adjacent_vertices.push(i.try_into().unwrap());
            }
        }
        
        return adjacent_vertices;
    }

    fn get_edge_weight(&mut self, v1: usize, v2: usize) -> Option<&u32> {
        // return self.matrix[v1][v2];
        let edge_weight = self.matrix.get(v1, v2);
        return edge_weight;
    }

    fn display(&mut self){
        for i in 0..=self.matrix.row_len().try_into().unwrap() { 
            for j in i+1..=self.matrix.column_len().try_into().unwrap() {
                if self.matrix.get(i, j) >= Some(&1) {
                    println!("{} -> {}", i, j);
                }
            }
       }
    }
    
    fn num_of_vertices(&mut self) -> usize {
        return self.num_vertices;
    }
}