use crate::graph::{Graph, Vertex, Weight};

static INF: i32 = 10000000;

fn first_min(matrix: &Vec<Vec<Weight>>, i: usize) -> Weight {
    let mut min = INF;
    let n = matrix.len();
    for k in 0..n {
        if matrix[i][k] < min && i != k {
            min = matrix[i][k];
        }
    }

    min
}

fn second_min(matrix: &Vec<Vec<Weight>>, i: usize) -> Weight {
    let mut first = INF;
    let mut second = INF;

    let n = matrix.len();
    for j in 0..n {
        if i == j {
            continue;
        }

        if matrix[i][j] <= first {
            second = first;
            first = matrix[i][j];
        } else if matrix[i][j] <= second && matrix[i][j] != first {
            second = matrix[i][j];
        }
    }

    second
}

fn m_rec(matrix: &Vec<Vec<Weight>>, v0: i32, w: Weight, level: usize, path: &mut Vec<i32>, visited: &mut Vec<bool>, final_res: &mut Weight, final_path: &mut Vec<i32>) {
    let n = matrix.len();

    if level == n {
        if matrix[path[level - 1] as usize][path[0] as usize] != 0 {
            let mut curr_res = w + matrix[path[level - 1] as usize][path[0] as usize];
            if curr_res < *final_res {
                *final_path = path.clone();
                final_path[n] = path[0];
                *final_res = curr_res;
            }
        }
        return;
    }

    for i in 0..n {
        if matrix[path[level - 1] as usize][i] != 0 && !visited[i] {
            let mut v0 = v0;
            let mut temp = v0;
            let mut w = w + matrix[path[level - 1] as usize][i];
            if level == 1 {
                v0 -= (first_min(matrix, path[level - 1] as usize) + first_min(matrix, i)) / 2;
            } else {
                v0 -= (second_min(matrix, path[level - 1] as usize) + first_min(matrix, i)) / 2;
            }
            if v0 + w < *final_res {
                path[level] = i as i32;
                visited[i] = true;
                m_rec(matrix, v0, w, level + 1, path, visited, final_res, final_path);
            }
            w -= matrix[path[level - 1]as usize][i];
            v0 = temp;

            for i in 0..visited.len() {
                visited[i] = false;
            }

            for j in 0..level {
                if path[j] != -1 {
                    visited[path[j] as usize] = true;
                }
            }

        }
    }
}

pub fn branch_and_bound(graph: &dyn Graph) -> (Weight, Vec<i32>) {
    let mut b = 0;
    let matrix = graph.adjacency_matrix();
    let mut curr_path = vec![-1; matrix.len() + 1];

    let mut visited = vec![false; matrix.len()];
    let mut final_path: Vec<i32> = vec![];
    let mut final_res: i32 = INF;

    for i in 0..matrix.len() {
        b += first_min(&matrix, i) + second_min(&matrix, i);
    }

    b = (b as f64 / 2.0).ceil() as i32;
    visited[0] = true;
    curr_path[0] = 0;
    m_rec(&matrix, b, 0, 1, &mut curr_path, &mut visited, &mut final_res, &mut final_path);

    (final_res, final_path)
}