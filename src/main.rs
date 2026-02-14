#![allow(unused)]

use itertools::{Itertools, iproduct};
use std::collections::HashMap;

// NOTE: bad, slow code incoming, my first time writing rust

#[derive(Debug, Clone)]
struct Graph {
    distances: Vec<Vec<u8>>, // shortest distance between node i and node j
}

fn peterson_graph() -> Graph {
    let mut distances: Vec<Vec<u8>> = vec![vec![0u8; 10]; 10];
    for (i, (a, b)) in (0..5).tuple_combinations().enumerate() {
        for (j, (c, d)) in (0..5).tuple_combinations().enumerate() {
            if a != c && a != d && b != c && b != d {
                distances[i][j] = 1;
                distances[j][i] = 1;
            } else if a == c && b == d || a == d && b == c {
                distances[i][j] = 0;
                distances[j][i] = 0;
            } else {
                distances[i][j] = 2;
                distances[j][i] = 2;
            }
        }
    }
    Graph { distances }
}

fn test_2_homogeneity(Graph { distances }: &Graph) -> bool {
    let n: usize = distances.len();
    for (a, b, c, d) in iproduct!((0..n), (0..n), (0..n), (0..n),) {
        if a != b && c != d && distances[a][b] == distances[c][d] {
            let codomain: Vec<usize> = (0..n).filter(|&x| x != a && x != b).collect();
            let domain: Vec<usize> = (0..n).filter(|&x| x != c && x != d).collect();
            let mut global_isometry_exists = false;
            for perm in domain.iter().cloned().permutations(domain.len()) {
                let mut sources = vec![a, b];
                sources.extend_from_slice(&codomain);
                let mut targets = vec![c, d];
                targets.extend_from_slice(&perm);
                let mut isometric = true;
                for (i, j) in (0..n).tuple_combinations() {
                    if distances[sources[i]][sources[j]] != distances[targets[i]][targets[j]] {
                        isometric = false;
                        break;
                    }
                }
                if isometric {
                    global_isometry_exists = true;
                    break;
                }
            }
            if !global_isometry_exists {
                println!("no global isometry for ({a}, {b}) -> ({c}, {d})");
                return false;
            }
        }
    }
    true
}

fn test_3_homogeneity(Graph { distances }: &Graph) -> bool {
    let n: usize = distances.len();
    for (a, b, c, d, e, f) in iproduct!((0..n), (0..n), (0..n), (0..n), (0..n), (0..n),) {
        if a != b
            && b != c
            && c != a
            && d != e
            && e != f
            && f != d
            && distances[a][b] == distances[d][e]
            && distances[b][c] == distances[e][f]
            && distances[c][a] == distances[f][d]
        {
            let codomain: Vec<usize> = (0..n).filter(|&x| x != a && x != b && x != c).collect();
            let domain: Vec<usize> = (0..n).filter(|&x| x != d && x != e && x != f).collect();
            let mut global_isometry_exists = false;
            for perm in domain.iter().cloned().permutations(domain.len()) {
                let mut sources = vec![a, b, c];
                sources.extend_from_slice(&codomain);
                let mut targets = vec![d, e, f];
                targets.extend_from_slice(&perm);
                let mut isometric = true;
                for (i, j) in (0..n).tuple_combinations() {
                    if distances[sources[i]][sources[j]] != distances[targets[i]][targets[j]] {
                        isometric = false;
                        break;
                    }
                }
                if isometric {
                    global_isometry_exists = true;
                    break;
                }
            }
            if !global_isometry_exists {
                println!("no global isometry for ({a}, {b}, {c}) -> ({d}, {e}, {f})");
                return false;
            }
        }
    }
    true
}

fn main() {
    let graph = peterson_graph();
    let homo2 = test_2_homogeneity(&graph);
    println!("2-homo for peterson is {homo2}");
    let homo3 = test_3_homogeneity(&graph);
    println!("3-homo for peterson is {homo3}");
    // takes around 11 seconds with optimization flags
    // about 10 times faster than equivalent python
}
