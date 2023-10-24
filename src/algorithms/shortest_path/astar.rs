use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::graph::{Map, MapCell};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
    priority: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type Heuristic = fn((usize, usize), (usize, usize)) -> usize;

pub fn astar(map: &Map, start: &MapCell, goal: &MapCell, heuristic: Heuristic) -> Option<(Vec<(usize, usize)>, usize)> {
    let mut count = 0 as usize;
    let mut open_set = BinaryHeap::new();
    open_set.push(Node {
        position: start.position(),
        cost: 0,
        priority: heuristic(start.position(), goal.position()),
    });

    let mut came_from = vec![vec![None; map.width()]; map.height()];

    let mut cost_so_far = vec![vec![None; map.width()]; map.height()]; // Хранит стоимость пути от начальной вершины до другой
    cost_so_far[start.position().0][start.position().1] = Some(0);

    while let Some(current) = open_set.pop() {
        if current.position == goal.position() { // Нашли конечную точку
            let mut path = vec![current.position];
            let mut current_position = current.position;
            while let Some(prev_position) = came_from[current_position.0][current_position.1] { // Восстанавливаем путь
                path.push(prev_position);
                current_position = prev_position;
            }
            path.reverse();
            return Some((path, count));
        }

        for n in map.neighbors(map.get(current.position.0, current.position.1)) {
            count += 1;

            let x = n.x;
            let y = n.y;

            let next_position = (x as usize, y as usize);
            let height_diff = (map.get_by_tuple(current.position).height - n.height).abs() as usize; // Перепад высоты
            let new_cost = current.cost + 1 + height_diff; // Новый путь(от начальной вершины)

            if cost_so_far[next_position.0][next_position.1].map_or(true, |cost| new_cost < cost) { // Стоимость пути до вершины еще не вычислена или ее можго обновить
                cost_so_far[next_position.0][next_position.1] = Some(new_cost);
                open_set.push(Node {
                    position: next_position,
                    cost: new_cost,
                    priority: new_cost + heuristic(next_position, goal.position()),
                });
                came_from[next_position.0][next_position.1] = Some(current.position);
            }
        }
    }

    None
}