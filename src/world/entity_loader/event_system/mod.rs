use enumerable::Enumerable;

// using System.Collections;
// using System.Collections.Generic;
// using UnityEngine;

// namespace GraphBaseEvent{
//     public abstract class GraphBase : MonoBehaviour
//     {
//         protected int numVertices;
//         protected bool directed;

//         public GraphBase(int numVertices, bool directed = false)
//         {
//             this.numVertices = numVertices;
//             this.directed = directed;
//         }

//         public abstract void AddEdge(int v1, int v2, int weight);

//         public abstract IEnumerable<int> GetAdjacentVertices(int v);

//         public abstract int GetEdgeWeight(int v1, int v2);

//         public abstract void Display();

//         public int NumOfVertices { get { return this.numVertices; } }
//     }
// }

pub trait GraphBaseEvent{
    pub fn add_edge(v1: i32, v2: i32, weight: i32);
    pub fn get_adj_vertices(v: i32) -> Enumerable;
    pub fn get_edge_weight(v1: i32, v2: i32) -> i32;
    pub fn display();
    pub fn num_of_vertices() -> i32;
}

struct AdjMatrixGraph{
    pub matrix: [i32]
}

impl GraphBaseEvent for AdjMatrixGraph{
    fn add_edge(v1: i32, v2: i32, weight: i32){

    }

    fn get_adj_vertices(v: i32) -> Enumerable{

    }

    fn get_edge_weight(v1: i32, v2: i32) -> i32{

    }

    fn display(){

    }
    
    fn num_of_vertices() -> i32{
        
    }
}