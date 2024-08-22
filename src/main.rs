use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn new() -> Player {
        Player { x: 0, y: 0 }
    }

    fn move_up(&mut self) {
        self.y += 1;
    }

    fn move_down(&mut self) {
        self.y -= 1;
    }

    fn move_left(&mut self) {
        self.x -= 1;
    }

    fn move_right(&mut self) {
        self.x += 1;
    }
}

enum Command {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    GetPosition,
    Execute(Box<dyn FnOnce() + Send>)
}


fn start_event_loop(receiver: mpsc::Receiver<Command>, sender: mpsc::Sender<(i32, i32)>) {
    let mut player = Player::new();

    loop {
        match receiver.recv() {
            Ok(Command::MoveUp) => {
                thread::sleep(std::time::Duration::from_secs(2));
                player.move_up();
                println!("Moved up: {:?}", player);
            }
            Ok(Command::MoveDown) => {
                thread::sleep(std::time::Duration::from_secs(2));
                player.move_down();
                println!("Moved down: {:?}", player);
            }
            Ok(Command::MoveLeft) => {
                thread::sleep(std::time::Duration::from_secs(2));
                player.move_left();
                println!("Moved left: {:?}", player);
            }
            Ok(Command::MoveRight) => {
                thread::sleep(std::time::Duration::from_secs(2));
                player.move_right();
                println!("Moved right: {:?}", player);
            }
            Ok(Command::GetPosition) => {
                let _ = sender.send((player.x, player.y));
            }
            Ok(Command::Execute(func)) =>{
                func();
            }
            Err(_) => {
                println!("error");
                break;
            }
        }
    }
}


fn main() {
    let (sender_cmd, receiver_cmd) = mpsc::channel();
    let (pos_sender, pos_receiver) = mpsc::channel();

    let event_loop_handle = thread::spawn(move || {
        start_event_loop(receiver_cmd, pos_sender);
    });

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "w" => sender_cmd.send(Command::MoveUp).unwrap(),
            "s" => sender_cmd.send(Command::MoveDown).unwrap(),
            "a" => sender_cmd.send(Command::MoveLeft).unwrap(),
            "d" => sender_cmd.send(Command::MoveRight).unwrap(),
            "f" => sender_cmd.send(Command::Execute(Box::new(move || {
                println!("Yohoho!")
            }))).unwrap(),
            "p" => {
                sender_cmd.send(Command::GetPosition).unwrap();
                let (x, y) = pos_receiver.recv().unwrap();
                println!("pos ({}, {})", x, y);
            }
            "q" => {
                println!("bye...");
                break;
            }
            _ => println!("Unknown; wasd p q"),
        }
    }

    drop(sender_cmd);
    event_loop_handle.join().unwrap(); 
}
