use std::vec;
struct LocationInMaze {
    row: usize,
    col: usize,
}

// 迷宫结构体
struct Maze {
    map: Vec<Vec<i32>>,
    player_location: LocationInMaze,
}

impl Maze {
    // 获得地图基底(边界为-1,内部网格内为1,网格上为0)
    fn new(map_level: usize) -> Self {
        let mut base: Vec<Vec<i32>> = vec![];
        for r in 0..map_level * 2 + 1 {
            let mut line: Vec<i32> = vec![];
            for c in 0..map_level * 2 + 1 {
                if r == 0 || c == 0 || r == map_level * 2 || c == map_level * 2 {
                    line.push(-1);
                } else if r % 2 != 0 && c % 2 != 0 {
                    line.push(1);
                } else {
                    line.push(0);
                }
            }
            base.push(line);
        }
        Maze {
            map: base,
            player_location: LocationInMaze { row: 1, col: 1 },
        }
    }

    // 将点的周围设为待定墙壁(2)
    fn set_wall_to_pending_of_cell(&mut self, r: usize, c: usize) {
        if self.map[r - 1][c] == 0 {
            self.map[r - 1][c] = 2;
        }
        if self.map[r + 1][c] == 0 {
            self.map[r + 1][c] = 2;
        }
        if self.map[r][c - 1] == 0 {
            self.map[r][c - 1] = 2;
        }
        if self.map[r][c + 1] == 0 {
            self.map[r][c + 1] = 2;
        }
    }

    // 设定迷宫开始延伸的起点
    fn set_start_cell(&mut self) {
        self.map[1][1] = 3;
        self.set_wall_to_pending_of_cell(1, 1);
    }

    // 循环停止判定（是否存在未判定的区域）
    fn need_generation(&self) -> bool {
        for line in &self.map {
            for item in line {
                if *item == 2 {
                    return true;
                }
            }
        }
        false
    }

    // 判断墙壁（如果相邻空单元（1）则打通（变为4），如果不相邻空单元则为墙壁（0））
    fn judging_the_wall(&mut self, r: usize, c: usize) {
        if (self.map[r - 1][c] == 3 || self.map[r - 1][c] == 5) && self.map[r + 1][c] == 1 {
            self.map[r][c] = 4;
            self.map[r + 1][c] = 3;
            self.set_wall_to_pending_of_cell(r + 1, c);
        } else if (self.map[r][c - 1] == 3 || self.map[r][c - 1] == 5) && self.map[r][c + 1] == 1 {
            self.map[r][c] = 4;
            self.map[r][c + 1] = 3;
            self.set_wall_to_pending_of_cell(r, c + 1);
        } else if (self.map[r + 1][c] == 3 || self.map[r + 1][c] == 5) && self.map[r - 1][c] == 1 {
            self.map[r][c] = 4;
            self.map[r - 1][c] = 3;
            self.set_wall_to_pending_of_cell(r - 1, c);
        } else if (self.map[r][c + 1] == 3 || self.map[r][c + 1] == 5) && self.map[r][c - 1] == 1 {
            self.map[r][c] = 4;
            self.map[r][c - 1] = 3;
            self.set_wall_to_pending_of_cell(r, c - 1);
        } else {
            self.map[r][c] = 0
        }
    }

    // 随机选择墙壁处理
    fn randomly_select_wall_treatment(&mut self) {
        let mut wall = 0;
        let map_len = self.map.len();
        for line in self.map.clone() {
            for cell in line {
                if cell == 2 {
                    wall += 1;
                }
            }
        }
        use rand::{thread_rng, Rng};
        let random_wall = thread_rng().gen_range(0..wall);
        wall = 0;
        'loopout: for r in 0..map_len {
            for c in 0..map_len {
                if self.map[r][c] == 2 {
                    if wall == random_wall {
                        self.judging_the_wall(r, c);
                        break 'loopout;
                    }
                    wall += 1;
                }
            }
        }
        // if need_generation(map) {
        //     // TODO start x/y = 6
        // }
    }

    // 生成地图
    pub fn generate_map(&mut self) {
        self.set_start_cell();
        loop {
            self.randomly_select_wall_treatment();
            if !self.need_generation() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array2d_is_avalble_and_mutable() {
        let mut base_maze = Maze::new(2);
        base_maze.map[2][2] = 10;
        assert_eq!(
            base_maze.map,
            vec![
                vec![-1, -1, -1, -1, -1],
                vec![-1, 1, 0, 1, -1],
                vec![-1, 0, 10, 0, -1],
                vec![-1, 1, 0, 1, -1],
                vec![-1, -1, -1, -1, -1],
            ]
        );
    }

    #[test]
    fn map_generation_completed() {
        let mut maze = Maze::new(5);
        println!("map_level is {}", 5);
        maze.generate_map();
        for line in maze.map {
            for cell in line {
                match cell {
                    -1 | 0 | 3 | 4 | 5 | 6 => {}
                    _ => {
                        panic!("map generation fail")
                    }
                }
            }
        }
    }
}
