use aoc_helpers::data_loader::DataLoader;
use aoc_helpers::matrix_helper::{MatrixLoader, get_flatten_matrix};
use simple_matrix::Matrix;

pub fn is_edge((row, col, _): &(usize, usize, &i32), mat: &Matrix<i32>) -> bool {
    *row == 0 || *col == 0 || *row == mat.rows() - 1 || *col == mat.cols() - 1
}

pub fn is_visible_in_row(item: &(usize, usize, &i32), mat: &Matrix<i32>) -> bool {
    let (row, col, val) = item;
    is_edge(item, mat)
        || mat.get_row(*row).unwrap().take(*col).max().unwrap() < val
        || mat.get_row(*row).unwrap().skip(*col + 1).max().unwrap() < val
}

pub fn is_visible_in_col(item: &(usize, usize, &i32), mat: &Matrix<i32>) -> bool {
    let (row, col, val) = item;
    is_edge(item, mat)
        || mat.get_col(*col).unwrap().take(*row).max().unwrap() < val
        || mat.get_col(*col).unwrap().skip(*row + 1).max().unwrap() < val
}

pub fn is_visible(item: &(usize, usize, &i32), mat: &Matrix<i32>) -> bool {
    is_visible_in_col(item, mat) || is_visible_in_row(item, mat)
}

fn get_score_of_line(line: &Vec<i32>, val: i32) -> i32 {
    let mut res = 0;
    for v in line.iter() {
        if *v < val {
            res += 1
        } else if *v >= val {
            res += 1;
            break;
        }
    }

    res
}

pub fn get_view_score(item: &(usize, usize, &i32), mat: &Matrix<i32>) -> i32 {
    let (row, col, val) = item;

    let left_tmp = mat.get_row(*row).unwrap().take(*col).collect::<Vec<&i32>>();
    let left_line = left_tmp
        .iter()
        .rev()
        .cloned()
        .cloned()
        .collect::<Vec<i32>>();
    let left = get_score_of_line(&left_line, *val.clone());

    let right = get_score_of_line(
        &mat.get_row(*row)
            .unwrap()
            .skip(*col + 1)
            .cloned()
            .collect::<Vec<i32>>(),
        *val.clone(),
    );

    let top_tmp = mat.get_col(*col).unwrap().take(*row).collect::<Vec<&i32>>();
    let top_line = top_tmp
        .iter()
        .rev()
        .cloned()
        .cloned()
        .collect::<Vec<i32>>();
    let top = get_score_of_line(&top_line, *val.clone());

    let bottom = get_score_of_line(
        &mat.get_col(*col)
            .unwrap()
            .skip(*row + 1)
            .cloned()
            .collect::<Vec<i32>>(),
        *val.clone(),
    );

    let res = left * right * top * bottom;
    res
}

pub fn solve_a(loader: &DataLoader) -> Result<String, &str> {
    let mat = loader.to_matrix::<i32>()?;
    let mat_iter = get_flatten_matrix(&mat);
    let count_vec = mat_iter
        .iter()
        .filter(|item| is_visible(item, &mat))
        .collect::<Vec<&(usize, usize, &i32)>>();
    let count = count_vec.iter().count();
    Ok(count.to_string())
}

pub fn solve_b(loader: &DataLoader) -> Result<String, &str> {
    let mat = loader.to_matrix::<i32>()?;
    let mat_iter = get_flatten_matrix(&mat);
    let max_view_score = mat_iter
        .iter()
        .map(|item| get_view_score(item, &mat))
        .max()
        .unwrap();
    Ok(max_view_score.to_string())
}

#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn test_is_edge() {
        let mat = DataLoader::from_file("data/test_input.txt")
            .to_matrix::<i32>()
            .unwrap();
        assert!(is_edge(&(0, 0, &0), &mat));
        assert!(is_edge(&(1, 0, &0), &mat));
        assert!(is_edge(&(4, 0, &0), &mat));
        assert!(is_edge(&(4, 0, &0), &mat));
        assert!(is_edge(&(4, 3, &0), &mat));
        assert!(is_edge(&(0, 4, &0), &mat));
        assert!(is_edge(&(1, 4, &0), &mat));

        assert!(!is_edge(&(1, 1, &0), &mat));
        assert!(!is_edge(&(2, 3, &0), &mat));
    }

    #[test]
    fn test_matrix_iter() {
        let mat = DataLoader::from_file("data/test_input.txt")
            .to_matrix::<i32>()
            .unwrap();

        assert_eq!(get_flatten_matrix(&mat).len(), 25);

        assert_eq!(
            get_flatten_matrix(&mat)
                .iter()
                .filter(|item| !is_edge(item, &mat))
                .count(),
            9
        );
    }

    #[test]
    fn test_is_visible() {
        let mat = DataLoader::from_file("data/test_input.txt")
            .to_matrix::<i32>()
            .unwrap();

        assert!(is_visible_in_row(&(1, 1, &5), &mat));
        assert!(is_visible_in_col(&(1, 1, &5), &mat));

        assert!(is_visible_in_row(&(1, 2, &5), &mat));
        assert!(is_visible_in_col(&(1, 2, &5), &mat));

        assert!(!is_visible_in_row(&(1, 3, &1), &mat));
        assert!(!is_visible_in_col(&(1, 3, &1), &mat));

        assert!(is_visible_in_row(&(2, 1, &5), &mat));
        assert!(!is_visible_in_col(&(2, 1, &5), &mat));

        assert!(is_visible_in_row(&(2, 3, &3), &mat));
        assert!(!is_visible_in_col(&(2, 3, &3), &mat));
    }
}
