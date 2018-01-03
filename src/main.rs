extern crate rand;

use rand::random;
use std::io::{self, Read};
use std::thread;
use std::time::Duration;

const HEIGHT: u8 = 20;
const WIDTH: u8 = 50;

struct Snake {
    direction: u8,
    fruit: (u8, u8),
    gameOver: bool,
    position: (u8, u8),
    score: u32
}

fn draw(snake: &Snake) {
    print!("{}[2J", 27 as char);

    for _i in 0..WIDTH+2 {
        print!("#");
    }

    print!("\n");

    for i in 0..HEIGHT {

        for j in 0..WIDTH {
            if j == 0 {
                print!("#");
            }

            if i == snake.position.1 && j == snake.position.0 {
                print!("O");
            } else if i == snake.fruit.1 && j == snake.fruit.0 {
                print!("F");
            } else {
                print!(" ");
                // TODO: Add logic for the tail
            }

            if j == WIDTH -1 {
                print!("#");
            }
        }

        print!("\n");
    }

    for _i in 0..WIDTH+2 {
        print!("#");
    }

    println!("\nScore: {}", snake.score);
    println!("Direction: {}", snake.direction);
}

fn input(snake: &mut Snake) {
    let mut input: Vec<u8> = vec![];

    io::stdin().read(&mut input);

    let mut iterator = input.iter();

    match iterator.next() {
        Some(&65) => snake.direction = 1,
        _ => ()
    }
}

fn logic() {

}

fn main() {
    let mut snake = Snake {
        direction: 0,
        fruit: (random::<u8>() % WIDTH, random::<u8>() % HEIGHT),
        gameOver: false,
        position: (WIDTH / 2, HEIGHT / 2),
        score: 0
    };

    while !snake.gameOver {
        draw(&snake);
        input(&mut snake);
        logic();
        thread::sleep(Duration::from_millis(60));
    }
}
