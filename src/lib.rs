use std::vec;

/// 迷宫位置结构体
pub struct LocationInMaze {
    pub row: usize,
    pub col: usize,
}

/// 迷宫单元类型
#[derive(Debug, PartialEq)]
pub enum CellType {
    /// 边界
    Border,
    /// 初始网格    
    Wall,
    /// 待定单元      
    PendingUnit,
    /// 待定墙壁
    PendingWall,
    /// 确定单元
    DetermineUnit,
    /// 联通网格
    ConnectedWall,
    /// 起点
    StartPoint,
    /// 终点
    EndPoint,
    /// 路径
    Path,
}

impl CellType {
    pub fn new(r: usize, c: usize, map_level: &usize) -> Self {
        if r == 0 || c == 0 || r == map_level * 2 || c == map_level * 2 {
            CellType::Border
        } else if r % 2 != 0 && c % 2 != 0 {
            CellType::PendingUnit
        } else {
            CellType::Wall
        }
    }
}

/// 迷宫结构体
pub struct Maze {
    map: Vec<Vec<CellType>>,
    pub player_location: LocationInMaze,
    pub map_level: usize,
}

impl Maze {
    /// 获得地图基底(边界为CellType::Border,内部网格内为CellType::PendingUnit,网格上为CellType::Wall)
    pub fn new(map_level: usize) -> Self {
        let mut base: Vec<Vec<CellType>> = vec![];
        for r in 0..map_level * 2 + 1 {
            let mut line: Vec<CellType> = vec![];
            for c in 0..map_level * 2 + 1 {
                line.push(CellType::new(r, c, &map_level))
            }
            base.push(line);
        }
        Maze {
            map: base,
            player_location: LocationInMaze { row: 1, col: 1 },
            map_level,
        }
    }

    /// 将点的周围设为待定墙壁(2)
    fn set_wall_to_pending_of_cell(&mut self, r: usize, c: usize) {
        if self.map[r - 1][c] == CellType::Wall {
            self.map[r - 1][c] = CellType::PendingWall;
        }
        if self.map[r + 1][c] == CellType::Wall {
            self.map[r + 1][c] = CellType::PendingWall;
        }
        if self.map[r][c - 1] == CellType::Wall {
            self.map[r][c - 1] = CellType::PendingWall;
        }
        if self.map[r][c + 1] == CellType::Wall {
            self.map[r][c + 1] = CellType::PendingWall;
        }
    }

    /// 设定迷宫开始延伸的起点
    fn set_start_cell(&mut self) {
        self.map[1][1] = CellType::DetermineUnit;
        self.set_wall_to_pending_of_cell(1, 1);
    }

    /// 循环停止判定（是否存在未判定的区域）
    fn need_generation(&self) -> bool {
        for line in &self.map {
            for item in line {
                if *item == CellType::PendingWall {
                    return true;
                }
            }
        }
        false
    }

    /// 判断墙壁（如果相邻空单元（1）则打通（变为CellType::ConnectedWall），如果不相邻空单元则为墙壁（CellType::Wall））
    fn judging_the_wall(&mut self, r: usize, c: usize) {
        if (self.map[r - 1][c] == CellType::DetermineUnit
            || self.map[r - 1][c] == CellType::StartPoint)
            && self.map[r + 1][c] == CellType::PendingUnit
        {
            self.map[r][c] = CellType::ConnectedWall;
            self.map[r + 1][c] = CellType::DetermineUnit;
            self.set_wall_to_pending_of_cell(r + 1, c);
        } else if (self.map[r][c - 1] == CellType::DetermineUnit
            || self.map[r][c - 1] == CellType::StartPoint)
            && self.map[r][c + 1] == CellType::PendingUnit
        {
            self.map[r][c] = CellType::ConnectedWall;
            self.map[r][c + 1] = CellType::DetermineUnit;
            self.set_wall_to_pending_of_cell(r, c + 1);
        } else if (self.map[r + 1][c] == CellType::DetermineUnit
            || self.map[r + 1][c] == CellType::StartPoint)
            && self.map[r - 1][c] == CellType::PendingUnit
        {
            self.map[r][c] = CellType::ConnectedWall;
            self.map[r - 1][c] = CellType::DetermineUnit;
            self.set_wall_to_pending_of_cell(r - 1, c);
        } else if (self.map[r][c + 1] == CellType::DetermineUnit
            || self.map[r][c + 1] == CellType::StartPoint)
            && self.map[r][c - 1] == CellType::PendingUnit
        {
            self.map[r][c] = CellType::ConnectedWall;
            self.map[r][c - 1] = CellType::DetermineUnit;
            self.set_wall_to_pending_of_cell(r, c - 1);
        } else {
            self.map[r][c] = CellType::Wall
        }
    }

    /// 随机选择墙壁处理
    fn randomly_select_wall_treatment(&mut self) {
        let mut wall = 0;
        let map_len = self.map.len();
        for line in &self.map {
            for cell in line {
                if *cell == CellType::PendingWall {
                    wall += 1;
                }
            }
        }
        use rand::{thread_rng, Rng};
        let random_wall = thread_rng().gen_range(0..wall);
        wall = 0;
        'loopout: for r in 0..map_len {
            for c in 0..map_len {
                if self.map[r][c] == CellType::PendingWall {
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

    /// 生成地图
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
        let base_maze = Maze::new(2);
        if base_maze.map
            != vec![
                vec![
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                ],
                vec![
                    CellType::Border,
                    CellType::PendingUnit,
                    CellType::Wall,
                    CellType::PendingUnit,
                    CellType::Border,
                ],
                vec![
                    CellType::Border,
                    CellType::Wall,
                    CellType::Wall,
                    CellType::Wall,
                    CellType::Border,
                ],
                vec![
                    CellType::Border,
                    CellType::PendingUnit,
                    CellType::Wall,
                    CellType::PendingUnit,
                    CellType::Border,
                ],
                vec![
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                    CellType::Border,
                ],
            ]
        {
            panic!("not eq!")
        }
    }

    #[test]
    fn map_generation_completed() {
        let mut maze = Maze::new(5);
        println!("map_level is {}", 5);
        maze.generate_map();
        for line in maze.map {
            for cell in line {
                match cell {
                    CellType::Border
                    | CellType::Wall
                    | CellType::DetermineUnit
                    | CellType::ConnectedWall
                    | CellType::StartPoint
                    | CellType::EndPoint => {}
                    _ => {
                        panic!("map generation fail")
                    }
                }
            }
        }
    }
}
