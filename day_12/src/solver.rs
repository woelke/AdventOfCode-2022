use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::{get_flatten_matrix, print_matrix, MatrixLoader};
use comparator::collections::BinaryHeap;
use comparator::comparing;
use simple_matrix::Matrix;
use std::iter;

#[derive(Copy, Clone, Debug)]
struct Pos(usize, usize);

impl Pos {
    fn row(&self) -> usize {
        self.0
    }

    fn col(&self) -> usize {
        self.1
    }

    fn in_boundary_of<T>(&self, mat: &Matrix<T>) -> bool {
        self.row() < mat.rows() && self.col() < mat.cols()
    }
}

fn find_poses(search_item: char, mat: &Matrix<char>) -> Vec<Pos> {
    get_flatten_matrix(mat)
        .iter()
        .filter(|(row, col, c)| **c == search_item)
        .map(|(row, col, c)| Pos(*row, *col))
        .collect::<Vec<Pos>>()
}

fn get_sorounding_poses(Pos(row, col): Pos) -> Vec<Pos> {
    let mut res: Vec<Pos> = vec![];

    res.push(Pos(row + 1, col));
    res.push(Pos(row, col + 1));

    if row > 0 {
        res.push(Pos(row - 1, col));
    }
    if col > 0 {
        res.push(Pos(row, col - 1));
    }

    res
}

#[derive(Debug)]
struct Job(u64, Pos);

impl Job {
    fn steps(&self) -> u64 {
        self.0
    }

    fn pos(&self) -> Pos {
        self.1
    }
}

fn calc_shortest_path(start: Pos, mat: &Matrix<char>) -> Matrix<u64> {
    let mut res: Matrix<u64> = Matrix::from_iter(mat.rows(), mat.cols(), iter::repeat(u64::MAX));
    let mut jobs = BinaryHeap::with_comparator(comparing(|job: &Job| u64::MAX - job.steps()));

    res.set(start.row(), start.col(), 0);
    jobs.push(Job(0, start));

    while let Some(job) = jobs.pop() {
        let next_step_count = job.steps() + 1;

        let next_poses = get_sorounding_poses(job.pos())
            .into_iter()
            .filter(|p| p.in_boundary_of(mat))
            .filter(|p| res.get(p.row(), p.col()).unwrap() > &next_step_count)
            .filter(|p| {
                (mat.get(p.row(), p.col()).unwrap().clone() as u8)
                    <= (mat.get(job.pos().row(), job.pos().col()).unwrap().clone() as u8) + 1
            })
            .collect::<Vec<Pos>>();

        for next_pos in next_poses {
            res.set(next_pos.row(), next_pos.col(), next_step_count);
            jobs.push(Job(next_step_count, next_pos));
        }
    }

    res
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mut mat: Matrix<char> = loader.to_matrix()?;

    let start = *find_poses('S', &mat).first().ok_or("Start not found")?;
    mat.set(start.row(), start.col(), 'a');

    let end = *find_poses('E', &mat).first().ok_or("End not found")?;
    mat.set(end.row(), end.col(), 'z');

    let res = calc_shortest_path(start, &mat);

    Ok(res.get(end.row(), end.col()).unwrap().to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mut mat: Matrix<char> = loader.to_matrix()?;

    {
    let tmp_start = *find_poses('S', &mat).first().ok_or("Start not found")?;
    mat.set(tmp_start.row(), tmp_start.col(), 'a');
    }

    let starts = find_poses('a', &mat);

    let end = *find_poses('E', &mat).first().ok_or("End not found")?;
    mat.set(end.row(), end.col(), 'z');

    let mut res : Vec<u64> = vec![];
    for start in starts {
        let res_mat = calc_shortest_path(start, &mat);
        let res_val = *res_mat.get(end.row(), end.col()).unwrap();
        res.push(res_val);
    }

    Ok(res.iter().min().unwrap().to_string())
}
