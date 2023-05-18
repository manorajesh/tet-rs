use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, KeyEventKind},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crate::tetrominoe::Tetrominoe;
use crate::{
    gamestate::GameState,
    tetrominoe::{State, TColor},
};

pub const EMP: char = '.';

pub fn render(gs: &mut GameState, is_updated: bool) {
    if !is_updated {
        return;
    }

    let mut stdout = stdout();
    let width: u16 = gs.display[0].len() as u16;

    stdout.queue(MoveTo(width + 3, 1)).unwrap(); // move cursor to top left
    for (c, row) in gs.display.iter().enumerate() {
        for ch in row {
            match ch.game_state {
                State::Empty => {
                    stdout.queue(Print("  ")).unwrap();
                }
                State::Active | State::Landed => {
                    stdout
                        .queue(SetForegroundColor(ch.as_color()))
                        .unwrap()
                        .queue(Print("██"))
                        .unwrap()
                        .queue(ResetColor)
                        .unwrap();
                }
                State::Ghost => {
                    stdout
                        .queue(SetForegroundColor(Color::Rgb {
                            r: 50,
                            g: 50,
                            b: 50,
                        }))
                        .unwrap()
                        .queue(Print("//"))
                        .unwrap()
                        .queue(ResetColor)
                        .unwrap();
                }
            }
        }
        stdout.queue(MoveTo(width + 3, (c + 2) as u16)).unwrap();
    }

    // hold piece
    stdout.queue(MoveTo(2, 1)).unwrap();
    stdout.queue(Print("Hold:")).unwrap();
    stdout.queue(MoveTo(2, 3)).unwrap();
    match &gs.hold_piece {
        Some(piece) => {
            let mut blank = Tetrominoe::new(None, None);
            let upright = blank.set(piece.ptype);
            for row in 0..upright.shape.len() {
                for col in 0..upright.shape[row].len() {
                    if upright.shape[row][col] == 'a' {
                        stdout
                            .queue(SetForegroundColor(piece.as_color()))
                            .unwrap()
                            .queue(Print("██"))
                            .unwrap()
                            .queue(ResetColor)
                            .unwrap();
                    } else {
                        stdout.queue(Print("  ")).unwrap();
                    }
                }
                stdout.queue(MoveTo(2, (row + 4) as u16)).unwrap();
            }
        }

        None => (),
    }

    // print stats
    stdout.queue(MoveTo(width * 4, 1)).unwrap();
    stdout
        .queue(Print(format!("Score: {}", gs.gamescore.score)))
        .unwrap();
    stdout.queue(MoveTo(width * 4, 3)).unwrap();
    stdout
        .queue(Print(format!("Level: {}", gs.gamescore.level)))
        .unwrap();
    stdout.queue(MoveTo(width * 4, 5)).unwrap();
    gs.gamescore.update();
    let time = gs.gamescore.get_time();
    stdout
        .queue(Print(format!("Time: {}:{:02}", time / 60, time % 60)))
        .unwrap();

    // next piece
    stdout.queue(MoveTo(width * 4, 8)).unwrap();
    stdout.queue(Print("Next:")).unwrap();
    stdout.queue(MoveTo(width * 4, 10)).unwrap();
    for row in 0..gs.next_piece.shape.len() {
        for col in 0..gs.next_piece.shape[row].len() {
            if gs.next_piece.shape[row][col] == 'a' {
                stdout
                    .queue(SetForegroundColor(gs.next_piece.as_color()))
                    .unwrap()
                    .queue(Print("██"))
                    .unwrap()
                    .queue(ResetColor)
                    .unwrap();
            } else {
                stdout.queue(Print("  ")).unwrap();
            }
        }
        stdout.queue(MoveTo(width * 4, (row + 11) as u16)).unwrap();
    }

    stdout.flush().unwrap();
}

pub fn init(width: usize, height: usize) -> Vec<Vec<Tetrominoe>> {
    let mut display: Vec<Vec<Tetrominoe>> = Vec::new();

    // generation
    for _ in 0..height {
        display.push(vec![Tetrominoe::default(); width]);
    }

    // walls
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();
    stdout.queue(MoveTo(11, 1)).unwrap(); // move cursor to top left while leaving space for hold
    for row in display.iter().enumerate() {
        stdout.queue(Print("<!")).unwrap(); // left wall
        for _ in row.1 {
            stdout.queue(Print("  ")).unwrap();
        }
        stdout.queue(Print("!>")).unwrap(); // right wall
        stdout.queue(MoveTo(11, (row.0 + 2) as u16)).unwrap();
    }
    stdout
        .queue(Print(format!(
            "<!{}!>\r\n",
            "=".repeat(display[0].len() * 2)
        )))
        .unwrap(); // bottom wall
    stdout
        .queue(Print(format!(
            "{}{}",
            " ".repeat(13),
            "\\/".repeat(display[0].len())
        )))
        .unwrap(); // bottom spikes
    stdout.queue(Hide).unwrap(); // Hide the cursor
    stdout.flush().unwrap();

    display
}

pub fn gravity(gs: &mut GameState) -> bool {
    let prev_display = gs.display.clone();
    for row in (0..gs.display.len()).rev() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col].game_state == State::Active {
                if row == gs.display.len() - 1
                    || gs.display[row + 1][col].game_state == State::Landed
                {
                    gs.display = prev_display;
                    landed(gs);
                    let game_over = new_piece(gs, None);
                    return game_over;
                }

                gs.display[row + 1][col] = gs.display[row][col];
                gs.display[row][col] = Tetrominoe::new(None, None);
            }
        }
    }
    gs.active_piece.row += 1;
    false
}

pub fn handle_input(gs: &mut GameState, key: char) {
    let prev_display = gs.display.clone();
    match key {
        'l' => {
            for row in (0..gs.display.len()).rev() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col].game_state == State::Active {
                        if col == 0 || gs.display[row][col - 1].game_state == State::Landed {
                            gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col - 1] = gs.display[row][col];
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            if gs.active_piece.col > 0 {
                gs.active_piece.col -= 1;
            }
        }

        'r' => {
            for row in (0..gs.display.len()).rev() {
                for col in (0..gs.display[row].len()).rev() {
                    if gs.display[row][col].game_state == State::Active {
                        if col == gs.display[row].len() - 1
                            || gs.display[row][col + 1].game_state == State::Landed
                        {
                            gs.display = prev_display;
                            return;
                        }
                        gs.display[row][col + 1] = gs.display[row][col];
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }
            gs.active_piece.col += 1;
        }

        's' => {
            // bring down piece until new piece is created
            while gs.display[0][gs.display[0].len() / 2].game_state == State::Empty {
                gravity(gs);
            }
        }

        'd' => {
            gravity(gs);
        }

        'u' => {
            // let prev_display = gs.display.clone();
            let prev_piece = gs.active_piece;

            // rotate piece
            gs.active_piece.rotate();
            if gs.active_piece.row + 4 > gs.display.len() {
                gs.active_piece.row = gs.display.len() - 4;
            }

            if gs.active_piece.col + 4 > gs.display[0].len() {
                gs.active_piece.col = gs.display[0].len() - 4;
            }

            // clear piece and replace with new rotated piece
            for row in 0..gs.display.len() {
                for col in 0..gs.display[row].len() {
                    if gs.display[row][col].game_state == State::Active {
                        gs.display[row][col] = Tetrominoe::new(None, None);
                    }
                }
            }

            for row in gs.active_piece.row..gs.active_piece.row + 4 {
                for col in gs.active_piece.col..gs.active_piece.col + 4 {
                    if gs.display[row][col].game_state == State::Landed {
                        gs.display = prev_display;
                        gs.active_piece = prev_piece;
                        return;
                    }

                    if gs.active_piece.shape[row - gs.active_piece.row][col - gs.active_piece.col]
                        == 'a'
                    {
                        gs.display[row][col] =
                            Tetrominoe::new(Some(State::Active), Some(gs.active_piece.color));
                    }
                }
            }
        }

        _ => (),
    }
}

pub fn new_piece(gs: &mut GameState, desired_piece: Option<char>) -> bool {
    let half_width = gs.display[0].len() / 2;

    // game over
    if gs.display[0][half_width].game_state != State::Empty {
        return true;
    }

    let piece = desired_piece.unwrap_or_else(|| get_next_piece(&mut gs.next_piece));
    match piece {
        'I' => {
            // I
            // I
            // I
            // I
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
            gs.display[3][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Cyan));
        }
        'J' => {
            //  J
            //  J
            // JJ
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
            gs.display[2][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Blue));
        }
        'L' => {
            // L
            // L
            // LL
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[2][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
            gs.display[2][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Orange));
        }
        'O' => {
            // OO
            // OO
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[0][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
            gs.display[1][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Yellow));
        }
        'S' => {
            // SS
            //  SS
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[0][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[1][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Green));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Green));
        }
        'T' => {
            // T
            // TT
            // T
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width - 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
            gs.display[1][half_width + 1] =
                Tetrominoe::new(Some(State::Active), Some(TColor::Magenta));
        }
        'Z' => {
            //  ZZ
            // ZZ
            gs.display[0][half_width - 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[0][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[1][half_width] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
            gs.display[1][half_width + 1] = Tetrominoe::new(Some(State::Active), Some(TColor::Red));
        }
        _ => panic!("unknown picece: {}", piece),
    }
    gs.active_piece.set(piece);
    gs.active_piece.set_pos(0, half_width - 1);
    false
}

pub fn landed(gs: &mut GameState) {
    for row in &mut gs.display {
        for ch in row {
            if ch.game_state == State::Active {
                ch.game_state = State::Landed;
            }
        }
    }
}

pub fn full_line(gs: &mut GameState) {
    let mut lines: usize = 0;
    'outer: for row in (0..gs.display.len()).rev() {
        for ch in &gs.display[row] {
            if ch.game_state != State::Landed {
                continue 'outer;
            }
        }
        gs.display.remove(row);
        lines += 1;
    }

    for _ in 0..lines {
        gs.display
            .insert(0, vec![Tetrominoe::default(); gs.display[0].len()]); // add new line at the top
    }

    match lines {
        1 => gs.gamescore.score += 40 * (gs.gamescore.level + 1),
        2 => gs.gamescore.score += 100 * (gs.gamescore.level + 1),
        3 => gs.gamescore.score += 300 * (gs.gamescore.level + 1),
        4 => gs.gamescore.score += 1200 * (gs.gamescore.level + 1),
        _ => (),
    }

    gs.gamescore.level = gs.gamescore.score / 1000;
}

pub fn ghost_piece(gs: &mut GameState) {
    for row in 0..gs.display.len() {
        for col in 0..gs.display[row].len() {
            if gs.display[row][col].game_state == State::Ghost {
                gs.display[row][col].game_state = State::Empty;
            }
        }
    }

    let mut ghost = gs.clone();

    gravity_until_new_piece(&mut ghost);

    for row in 0..ghost.display.len() {
        for col in 0..ghost.display[row].len() {
            if ghost.display[row][col].game_state == State::Active
                && gs.display[row][col].game_state == State::Empty
            {
                gs.display[row][col].game_state = State::Ghost;
            }
        }
    }
}

fn gravity_until_new_piece(gs: &mut GameState) {
    let mut prev_display = gs.display.clone();
    gravity(gs);
    while gs.display[0][gs.display[0].len() / 2].game_state == State::Empty {
        prev_display = gs.display.clone();
        gravity(gs);
    }
    gs.display = prev_display;
}

pub fn get_input() -> char {
    loop {
        if poll(Duration::from_millis(0)).unwrap() {
            let input = event::read().unwrap();
            match input {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'q', // quit
                Event::Key(KeyEvent {
                    code: KeyCode::Char(' '),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 's', // hard drop
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'c', // hold
                Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'p', // pause
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'l', // move left
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'r', // move right
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'u', // rotate clockwise
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'd', // soft drop
                Event::Key(KeyEvent {
                    code: KeyCode::Char('y'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'y', // yes
                Event::Key(KeyEvent {
                    code: KeyCode::Char('n'),
                    kind: KeyEventKind::Press,
                    ..
                }) => return 'n', // no

                _ => (),
            }
        } else {
            return ' ';
        }
    }
}

pub fn hold(gs: &mut GameState) {
    // clear piece
    for row in gs.display.iter_mut() {
        for col in row.iter_mut() {
            if col.game_state == State::Active {
                col.game_state = State::Empty;
            }
        }
    }

    // hold piece
    if let Some(hold) = &gs.hold_piece {
        let prev_piece = gs.active_piece;
        new_piece(gs, Some(hold.ptype));
        gs.hold_piece = Some(prev_piece);
    } else {
        gs.hold_piece = Some(gs.active_piece);
        new_piece(gs, None);
    }
}

fn get_next_piece(next_piece: &mut Tetrominoe) -> char {
    let temp = next_piece.ptype;
    *next_piece = Tetrominoe::random();
    temp
}

pub fn put_text(width: u16, height: u16, text: &str) {
    let mut stdout = stdout();

    // top bar
    stdout.queue(MoveTo(width + 3, height / 2 - 2)).unwrap();
    stdout
        .queue(SetForegroundColor(Color::Rgb {
            r: 255,
            g: 105,
            b: 97,
        }))
        .unwrap()
        .queue(Print("=".repeat(width as usize * 2)))
        .unwrap()
        .queue(ResetColor)
        .unwrap();

    stdout.queue(MoveTo(width + 3, height / 2 - 1)).unwrap();
    stdout.queue(Print(" ".repeat(width as usize * 2))).unwrap();

    // text
    stdout.queue(MoveTo(width + 3, height / 2)).unwrap();
    stdout
        .queue(SetForegroundColor(Color::Rgb {
            r: 255,
            g: 105,
            b: 97,
        }))
        .unwrap()
        .queue(Print(format!(
            "{:^text_width$}",
            text,
            text_width = width as usize * 2
        )))
        .unwrap()
        .queue(ResetColor)
        .unwrap();

    stdout.queue(MoveTo(width + 3, height / 2 + 1)).unwrap();
    stdout.queue(Print(" ".repeat(width as usize * 2))).unwrap();

    // bottom bar
    stdout.queue(MoveTo(width + 3, height / 2 + 2)).unwrap();
    stdout
        .queue(SetForegroundColor(Color::Rgb {
            r: 255,
            g: 105,
            b: 97,
        }))
        .unwrap()
        .queue(Print("=".repeat(width as usize * 2)))
        .unwrap()
        .queue(ResetColor)
        .unwrap();

    stdout.flush().unwrap();
}
