use bevy::log::info;
use array2d::{Array2D, Error};
use std::vec;
use std::convert::TryFrom;

/*
public class AdjacencyMatrixGraph : GraphBase{
    int[,] Matrix;
    
    public AdjacencyMatrixGraph(int numVertices, bool directed = false) : base(numVertices, directed)
    {
        GenerateEmptyMatrix(numVertices);
    }

    // public void Start() {
    //     AdjacencyMatrixGraph adjMatrixGraph = new AdjacencyMatrixGraph(9, false);
    //     adjMatrixGraph.AddEdge(0, 8);
    //     adjMatrixGraph.AddEdge(0, 3);
    //     adjMatrixGraph.AddEdge(8, 4);
    //     adjMatrixGraph.AddEdge(8, 3);
    //     // adjMatrixGraph.GetAdjacentVertices(8);
    //     adjMatrixGraph.Display();
    // }

    private void GenerateEmptyMatrix(int numVertices)
    {
        this.Matrix = new int[numVertices, numVertices];
        for (int row = 0; row < numVertices; row++)
        {
            for (int col = 0; col < numVertices; col++)
            {
                Matrix[row, col] = 0;
            }
        }
    }

    public override IEnumerable<int> GetAdjacentVertices(int v)
    {
        if (v < 0 || v >= this.numVertices) throw new ArgumentOutOfRangeException("Cannot access vertex");

        List<int> adjacentVertices = new List<int>();
        for (int i = 0; i < this.numVertices; i++)
        {
            if (this.Matrix[v, i] > 0)
                adjacentVertices.Add(i);
        }
        return adjacentVertices;
    }

    public override void AddEdge(int v1, int v2, int weight = 1){
        if (v1 >= this.numVertices || v2 >= this.numVertices || v1 < 0 || v2 < 0)
        throw new ArgumentOutOfRangeException("Vertices are out of bounds");

        if (weight < 1) throw new ArgumentException("Weight cannot be less than 1");

        this.Matrix[v1, v2] = weight;

        //In an undirected graph all edges are bi-directional
        if (!this.directed) 
            this.Matrix[v2, v1] = weight;
    }

    public override int GetEdgeWeight(int v1, int v2)
    {
        return this.Matrix[v1, v2];
    }

    public override void Display()
    {
       for(int i = 0; i < this.Matrix.GetLength(0); i++){ 
            for(int j = i+1; j < this.Matrix.GetLength(1); j++){
                if(this.Matrix[i,j] == 1){
                    Debug.Log(i+"->"+j);
                }
            }
       }
    }
}
*/

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
        //let _ = self.matrix.set(v1, v2, weight);

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
        for i in 0..=self.matrix.num_rows().try_into().unwrap() { 
            for j in i+1..=self.matrix.num_columns().try_into().unwrap() {
                //if self.matrix[i][j] == 1 {
                if self.matrix.get(i, j).is_some() {
                    println!("{} -> {}", i, j);
                }
            }
       }
    }
    
    fn num_of_vertices(&mut self) -> usize {
        return self.num_vertices;
    }
}
