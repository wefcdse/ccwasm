use std::{
    cell::Cell,
    collections::{HashSet, VecDeque},
};

use cc_wasm_api::utils::SyncNonSync;
/// run with `cargo run --example minesweeper`
///
/// in minecraft:
/// place a monitor on the top of a computer
/// run `ws_control p1 14111`
use rand::random;
use stupid_utils::{prelude::MutableInit, select::DotSelect};

use crate::{
    local_monitor::LocalMonitor,
    utils::{AsIfPixel, ColorId, Direction},
    vec2d::Vec2d,
    CLICKED,
};

const CLEAR_COLOR: ColorId = ColorId::White;
const TEXT_COLOR: ColorId = ColorId::Yellow;
const TEXT_BACKGROUND: ColorId = ColorId::Black;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameState {
    StartUp,
    JustStarted { x: usize, y: usize },
    Running(GameRunningState),
    Failed(GameRunningState),
    Successed(GameRunningState),
    Invalid,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameRunningState {
    size_x: usize,
    size_y: usize,
    field: Vec2d<Block>,
    uncovered: Vec2d<bool>,
    marked: Vec2d<bool>,
    selected_tool: Tool,
    total_mines: usize,
}
impl GameState {
    fn take_grs(&mut self) -> Option<GameRunningState> {
        let mut old = Self::Invalid;
        std::mem::swap(self, &mut old);
        match old {
            GameState::Running(grs) => Some(grs),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tool {
    Mark,
    SafeMark,
    Uncover,
}

impl Tool {
    pub fn switch(&mut self) {
        match self {
            Tool::Mark => *self = Tool::SafeMark,
            Tool::SafeMark => *self = Tool::Uncover,
            Tool::Uncover => *self = Tool::Mark,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Block {
    Mine,
    Safe,
    Surround(u8),
}

// async fn tick(state: &mut (LocalMonitor, GameState), _dt: Duration) {
//     let (monitor, game) = state;
//     let mut p1 = ports.get_port("p1").to_errors_result()?;

//     let (size_x, size_y) = p1
//         .monitor_get_size(MONITOR_SIDE)
//         .await?
//         .to_errors_result()?;
//     if size_x != monitor.x() || size_y != monitor.y() {
//         monitor.resize(size_x, size_y, AsIfPixel::colored_whitespace(CLEAR_COLOR));
//         *game = GameState::StartUp;
//     }

//     game_logic(game, monitor, &mut p1).await?;

//     monitor.sync(MONITOR_SIDE, &mut p1).await?;
//     Ok(())
// }

const BORDER_COLOR: ColorId = ColorId::Green;
const BORDER_COLOR_FAIL: ColorId = ColorId::Red;
const BORDER_COLOR_SUCCESS: ColorId = ColorId::Yellow;
static MINE_CHANCE: SyncNonSync<Cell<usize>> = SyncNonSync(Cell::new(10));

pub async fn game_logic(game: &mut GameState, monitor: &mut LocalMonitor) {
    'm: {
        match game {
            GameState::Invalid => {
                panic!()
            }
            GameState::StartUp => {
                {
                    let start = "start";
                    let (size_x, size_y) = monitor.size();
                    let starty = size_y / 2 + 1;
                    let startx = (size_x.max(5) - 5) / 2 + 1;
                    monitor.clear_with(CLEAR_COLOR);
                    monitor.write_str(
                        startx,
                        starty,
                        Direction::PosX,
                        start,
                        TEXT_BACKGROUND,
                        TEXT_COLOR,
                    );
                }
                let (left, right) = {
                    let diff = format!("{}", MINE_CHANCE.get());
                    let l = diff.len();
                    let (size_x, size_y) = monitor.size();
                    let starty = size_y;
                    let startx = (size_x.max(l) - l) / 2 + 1;
                    monitor.write_str(
                        startx,
                        starty,
                        Direction::PosX,
                        &diff,
                        ColorId::Lime,
                        TEXT_COLOR,
                    );
                    let left = startx - 2;
                    let right = startx + l + 1;
                    monitor.write_str(
                        left,
                        starty,
                        Direction::PosX,
                        "<",
                        ColorId::Lime,
                        TEXT_COLOR,
                    );
                    monitor.write_str(
                        right,
                        starty,
                        Direction::PosX,
                        ">",
                        ColorId::Lime,
                        TEXT_COLOR,
                    );
                    (left, right)
                };
                // MINE_CHANCE.set(MINE_CHANCE.get() + 1);
                if let Some((x, y)) = CLICKED.try_get() {
                    if y == monitor.y().try_into().unwrap() {
                        if x <= left.try_into().unwrap() {
                            // panic!();
                            let old = MINE_CHANCE.get();
                            MINE_CHANCE.set(((old * 5) / 6).max(2));
                            // MINE_CHANCE.set(1);
                        }
                        if x >= right.try_into().unwrap() {
                            let old = MINE_CHANCE.get();
                            MINE_CHANCE.set((old + (old / 4).max(1)).max(2));
                        }
                    } else {
                        monitor.clear_with(CLEAR_COLOR);

                        for x in 1..=monitor.x() {
                            monitor.write(x, 1, AsIfPixel::colored_whitespace(BORDER_COLOR));
                            monitor.write(
                                x,
                                monitor.y(),
                                AsIfPixel::colored_whitespace(BORDER_COLOR),
                            );
                        }

                        for y in 1..=monitor.y() {
                            monitor.write(1, y, AsIfPixel::colored_whitespace(BORDER_COLOR));
                            monitor.write(
                                monitor.x(),
                                y,
                                AsIfPixel::colored_whitespace(BORDER_COLOR),
                            );
                        }

                        *game = GameState::JustStarted {
                            x: (monitor.x() - 2),
                            y: (monitor.y() - 2),
                        };
                    }
                }
            }
            GameState::JustStarted {
                x: ref size_x,
                y: ref size_y,
            } => {
                let (click_x, click_y) = if let Some((x, y)) = CLICKED.try_get() {
                    if x >= 2
                        && y >= 2
                        && x - 2 < (*size_x as u16).into()
                        && y - 2 < (*size_y as u16).into()
                    {
                        (x - 2, y - 2)
                    } else {
                        break 'm;
                    }
                } else {
                    break 'm;
                };
                let (click_x, click_y) = (click_x as usize, click_y as usize);

                let field =
                    Vec2d::new_filled_copy(*size_x, *size_y, Block::Safe).mutable_init(|vec| {
                        vec.iter_mut().for_each(|(_, b)| {
                            let rand = random::<usize>() % MINE_CHANCE.get() == 0;
                            *b = rand.select(Block::Mine, Block::Safe);
                        });

                        vec[(click_x, click_y)] = Block::Safe;

                        for (x, y) in vec.iter_index() {
                            if let Block::Safe = vec[x][y] {
                                let dxdy = [
                                    (1, -1),
                                    (1, 0),
                                    (1, 1),
                                    (0, -1),
                                    (0, 1),
                                    (-1, -1),
                                    (-1, 0),
                                    (-1, 1),
                                ];
                                let mut mine_count = 0;

                                {
                                    let (x, y) = (x as isize, y as isize);
                                    for (dx, dy) in dxdy {
                                        let x = x + dx;
                                        let y = y + dy;
                                        mine_count += if x >= 0
                                            && y >= 0
                                            && x < vec.x() as isize
                                            && y < vec.y() as isize
                                        {
                                            (Block::Mine == vec[(x as usize, y as usize)])
                                                .select(1, 0)
                                        } else {
                                            0
                                        };
                                    }
                                }
                                if mine_count != 0 {
                                    vec[x][y] = Block::Surround(mine_count);
                                }
                            }
                        }
                    });

                let mine_count = field.iter().filter(|(_, b)| **b == Block::Mine).count();
                // dbg!(mine_count);
                let mut uncovered: Vec2d<bool> = Vec2d::new_filled(*size_x, *size_y, false)
                    .mutable_init(|v| {
                        v[(click_x, click_y)] = true;
                    });
                let marked: Vec2d<bool> = Vec2d::new_filled(*size_x, *size_y, false);

                // game logic
                process_uncover(click_x, click_y, &mut uncovered, &marked, &field);

                display_to_monitor(monitor, &uncovered, &marked, &field, false);
                monitor.write_str(
                    1,
                    monitor.y(),
                    Direction::PosX,
                    "uncover, click to switch mode",
                    BORDER_COLOR,
                    TEXT_COLOR,
                );

                monitor.write_str(
                    1,
                    1,
                    Direction::PosX,
                    &format!("mine:{}             ", mine_count),
                    BORDER_COLOR,
                    TEXT_COLOR,
                );

                *game = GameState::Running(GameRunningState {
                    size_x: *size_x,
                    size_y: *size_y,
                    field,
                    uncovered,
                    marked,
                    selected_tool: Tool::Uncover,
                    total_mines: mine_count,
                })
            }
            GameState::Running(grs) => {
                let (click_x, click_y) = if let Some((x, y)) = CLICKED.try_get() {
                    if x >= 2
                        && y >= 2
                        && x - 2 < (grs.size_x as u16).into()
                        && y - 2 < (grs.size_y as u16).into()
                    {
                        (x - 2, y - 2)
                    } else {
                        if y as usize == monitor.y() {
                            grs.selected_tool.switch();
                            match grs.selected_tool {
                                Tool::Mark => monitor.write_str(
                                    1,
                                    monitor.y(),
                                    Direction::PosX,
                                    "mark, click to switch mode     ",
                                    BORDER_COLOR,
                                    TEXT_COLOR,
                                ),
                                Tool::Uncover => monitor.write_str(
                                    1,
                                    monitor.y(),
                                    Direction::PosX,
                                    "uncover, click to switch mode     ",
                                    BORDER_COLOR,
                                    TEXT_COLOR,
                                ),
                                Tool::SafeMark => monitor.write_str(
                                    1,
                                    monitor.y(),
                                    Direction::PosX,
                                    "safe mark, click to switch mode     ",
                                    BORDER_COLOR,
                                    TEXT_COLOR,
                                ),
                            }
                        }
                        break 'm;
                    }
                } else {
                    break 'm;
                };
                let (click_x, click_y) = (click_x as usize, click_y as usize);

                let failed = running(grs, click_x, click_y);
                // dbg!(failed);

                let mine_count = {
                    let marked = grs.marked.iter().filter(|(_, b)| **b).count() as isize;
                    grs.total_mines as isize - marked
                };
                monitor.write_str(
                    1,
                    1,
                    Direction::PosX,
                    &format!("mine:{}             ", mine_count),
                    BORDER_COLOR,
                    TEXT_COLOR,
                );
                display_to_monitor(monitor, &grs.uncovered, &grs.marked, &grs.field, false);

                if failed {
                    let grs = game.take_grs().unwrap();
                    *game = GameState::Failed(grs);
                    break 'm;
                } else {
                    let unknown = grs.uncovered.iter().filter(|(_, u)| !**u).count();
                    if unknown == grs.total_mines {
                        let grs = game.take_grs().unwrap();
                        *game = GameState::Successed(grs);
                    }
                }
            }
            GameState::Failed(grs) => {
                for x in 1..=monitor.x() {
                    monitor.write(x, 1, AsIfPixel::colored_whitespace(BORDER_COLOR_FAIL));
                    monitor.write(
                        x,
                        monitor.y(),
                        AsIfPixel::colored_whitespace(BORDER_COLOR_FAIL),
                    );
                }

                for y in 1..=monitor.y() {
                    monitor.write(1, y, AsIfPixel::colored_whitespace(BORDER_COLOR_FAIL));
                    monitor.write(
                        monitor.x(),
                        y,
                        AsIfPixel::colored_whitespace(BORDER_COLOR_FAIL),
                    );
                }

                monitor.write_str(
                    1,
                    1,
                    Direction::PosX,
                    "Failed!!!!",
                    BORDER_COLOR_FAIL,
                    TEXT_COLOR,
                );

                grs.field.iter_index().for_each(|pos| {
                    let marked = grs.marked[pos];
                    // let uncovered = grs.uncovered[pos];
                    let block = grs.field[pos];
                    if block == Block::Mine && !marked {
                        grs.uncovered[pos] = true;
                    }
                });
                display_to_monitor(monitor, &grs.uncovered, &grs.marked, &grs.field, true);

                if CLICKED.try_get().is_some() {
                    *game = GameState::StartUp;
                }
            }
            GameState::Successed(_grs) => {
                for x in 1..=monitor.x() {
                    monitor.write(x, 1, AsIfPixel::colored_whitespace(BORDER_COLOR_SUCCESS));
                    monitor.write(
                        x,
                        monitor.y(),
                        AsIfPixel::colored_whitespace(BORDER_COLOR_SUCCESS),
                    );
                }

                for y in 1..=monitor.y() {
                    monitor.write(1, y, AsIfPixel::colored_whitespace(BORDER_COLOR_SUCCESS));
                    monitor.write(
                        monitor.x(),
                        y,
                        AsIfPixel::colored_whitespace(BORDER_COLOR_SUCCESS),
                    );
                }

                monitor.write_str(
                    1,
                    1,
                    Direction::PosX,
                    "Success!!!!",
                    BORDER_COLOR_SUCCESS,
                    ColorId::Black,
                );
                monitor.write_str(
                    1,
                    monitor.y(),
                    Direction::PosX,
                    "click to start new                 ",
                    BORDER_COLOR_SUCCESS,
                    ColorId::Black,
                );

                if let Some((_, y)) = CLICKED.try_get() {
                    if y == monitor.y().try_into().unwrap() {
                        *game = GameState::StartUp;
                    }
                }
            }
        }
    }
}

const MARKED: AsIfPixel = if let Some(p) = AsIfPixel::new('#', ColorId::Red, ColorId::White) {
    p
} else {
    panic!()
};

const WRONG_MARKED: AsIfPixel = if let Some(p) = AsIfPixel::new('#', ColorId::Black, ColorId::White)
{
    p
} else {
    panic!()
};

const MINE: AsIfPixel = if let Some(p) = AsIfPixel::new(' ', ColorId::Red, ColorId::White) {
    p
} else {
    panic!()
};
const SURROUND_COLOR: ColorId = ColorId::Magenta;
const SURROUND_COLOR_TEXT: ColorId = ColorId::White;
const SAFE_COLOR: ColorId = ColorId::LightBlue;

fn display_to_monitor(
    monitor: &mut LocalMonitor,
    uncovered: &Vec2d<bool>,
    marked: &Vec2d<bool>,
    field: &Vec2d<Block>,
    end: bool,
) {
    for (pos, uncovered) in uncovered.iter() {
        let (x, y) = pos;
        let (x, y) = (x + 2, y + 2);
        let marked = marked[pos];
        let mine = field[pos];

        if end && marked && mine != Block::Mine {
            monitor.write(x, y, WRONG_MARKED);
            continue;
        }

        if marked {
            monitor.write(x, y, MARKED);
            continue;
        }

        if !uncovered {
            monitor.write(x, y, AsIfPixel::colored_whitespace(CLEAR_COLOR));
            continue;
        }

        match mine {
            Block::Mine => monitor.write(x, y, MINE),
            Block::Safe => monitor.write(x, y, AsIfPixel::colored_whitespace(SAFE_COLOR)),
            Block::Surround(num) => monitor.write(
                x,
                y,
                AsIfPixel::new(
                    char::from_digit(num as u32, 10).unwrap(),
                    SURROUND_COLOR,
                    SURROUND_COLOR_TEXT,
                )
                .unwrap(),
            ),
        };
    }
}

/// bool: hit mine
fn process_uncover(
    x: usize,
    y: usize,
    uncovered: &mut Vec2d<bool>,
    marked: &Vec2d<bool>,
    field: &Vec2d<Block>,
) -> bool {
    let (size_x, size_y) = field.size();
    let dxdy = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    // uncover all surround blocks
    let process = |x, y, uncovered: &mut Vec2d<bool>| {
        let mut block_to_visit = VecDeque::new();
        let mut visited = HashSet::new();
        block_to_visit.push_back((x, y));
        visited.insert((x, y));
        while let Some((x, y)) = block_to_visit.pop_front() {
            uncovered[(x, y)] = true;
            // dbg!((x, y));
            if field[(x, y)] != Block::Safe {
                continue;
            }
            for (dx, dy) in dxdy {
                let pos = {
                    let (x1, y1) = (x as isize + dx, y as isize + dy);
                    if (0..size_x as isize).contains(&x1) && (0..size_y as isize).contains(&y1) {
                        (x1 as usize, y1 as usize)
                    } else {
                        continue;
                    }
                };
                let block = field[pos];
                let uncovered = uncovered[pos];
                let marked = marked[pos];
                if block != Block::Mine && !uncovered && !marked && !visited.contains(&pos) {
                    block_to_visit.push_back(pos);

                    visited.insert(pos);
                }
            }
        }
        // *uncovered = marked.clone();
    };

    if marked[(x, y)] {
        return false;
    }

    match field[(x, y)] {
        Block::Mine => true,
        Block::Safe => {
            process(x, y, uncovered);
            false
        }
        Block::Surround(_num) => {
            let mut remained = 8;
            for (dx, dy) in dxdy {
                let pos = {
                    let (x1, y1) = (x as isize + dx, y as isize + dy);
                    if (0..size_x as isize).contains(&x1) && (0..size_y as isize).contains(&y1) {
                    } else {
                        continue;
                    };
                    (x1 as usize, y1 as usize)
                };

                let uncovered = uncovered[pos];
                let marked = marked[pos];
                if marked || uncovered {
                    remained -= 1;
                    continue;
                }
            }

            if remained < 0 {
                return true;
            }

            process(x, y, uncovered);
            false
        }
    }
}

/// returns: failed
fn running(game: &mut GameRunningState, click_x: usize, click_y: usize) -> bool {
    match game.selected_tool {
        Tool::Mark => mark(game, click_x, click_y),
        Tool::Uncover => process_uncover(
            click_x,
            click_y,
            &mut game.uncovered,
            &game.marked,
            &game.field,
        ),
        Tool::SafeMark => safe_mark(game, click_x, click_y),
    }
}

fn safe_mark(game: &mut GameRunningState, click_x: usize, click_y: usize) -> bool {
    if !game.uncovered[(click_x, click_y)] {
        false
    } else {
        mark(game, click_x, click_y)
    }
}

fn mark(game: &mut GameRunningState, click_x: usize, click_y: usize) -> bool {
    if !game.uncovered[(click_x, click_y)] {
        game.marked[(click_x, click_y)] = !game.marked[(click_x, click_y)];
        return false;
    }
    let dxdy = [
        (1, -1),
        (1, 0),
        (1, 1),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    let num = if let Block::Surround(num) = game.field[click_x][click_y] {
        num
    } else {
        return false;
    };

    let mut mark_count = 0;
    let mut uncovered_count = 0;
    let mut vaild_block = 0;
    for (dx, dy) in dxdy {
        let pos = {
            let (x1, y1) = (click_x as isize + dx, click_y as isize + dy);
            if (0..game.size_x as isize).contains(&x1) && (0..game.size_y as isize).contains(&y1) {
            } else {
                continue;
            };
            (x1 as usize, y1 as usize)
        };
        if game.marked[pos] {
            mark_count += 1;
        }
        if game.uncovered[pos] {
            uncovered_count += 1;
        }
        vaild_block += 1;
    }
    let unknown_count = vaild_block - uncovered_count;

    if unknown_count == num {
        for (dx, dy) in dxdy {
            let pos = {
                let (x1, y1) = (click_x as isize + dx, click_y as isize + dy);
                if (0..game.size_x as isize).contains(&x1)
                    && (0..game.size_y as isize).contains(&y1)
                {
                } else {
                    continue;
                };
                (x1 as usize, y1 as usize)
            };

            if !game.uncovered[pos] {
                game.marked[pos] = true;
            }
        }
    }

    let mut failed = false;
    if mark_count == num {
        for (dx, dy) in dxdy {
            let pos = {
                let (x1, y1) = (click_x as isize + dx, click_y as isize + dy);
                if (0..game.size_x as isize).contains(&x1)
                    && (0..game.size_y as isize).contains(&y1)
                {
                } else {
                    continue;
                };
                (x1 as usize, y1 as usize)
            };
            if process_uncover(pos.0, pos.1, &mut game.uncovered, &game.marked, &game.field) {
                failed = true;
            }
        }
    }

    failed
}
