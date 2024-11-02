use std::{io,println};


#[derive(Debug)]
struct Question {
    text: String,
    options: Vec<String>,
    answer: String,
    prize: i32
}

#[derive(Debug)]
struct Game {
    balance: i32,
    questions: Vec<Question>,
}


impl Game {
    pub fn run(&mut self) {
        for q in &self.questions {
            println!("{:?}", q.text);
            println!("{:?}", q.options);
            println!("Choose the correct option");

            let mut option =  String::new();
            io::stdin()
                .read_line(&mut option)
                .expect("failed to read the option selected");

            println!("the answer selectd is: {}", option);
            println!("the correct is: {}", q.answer);

            if option.trim() != q.answer {
                println!("You loose all your rewards :(, try again");
                break;
            }

            self.balance += q.prize;
            println!("your current reward is:{}", self.balance);


            println!("Would you like to continue or stop here? (y/n)");


            let mut response = String::new();
            io::stdin().read_line(&mut response).expect("failed to read the  answer");
            println!("{}", response); 
           

            if response.trim() == "n".to_string() {
                println!("You earned:{}", self.balance);
                break;
            }
        }
    }
}

fn main() {
    let options = vec![
        String::from("a. Petro"),
        String::from("b. Uribe"),
        String::from("c. Shiky"),
        String::from("d. Shakira"),
    ];


    let question_a = Question {
        text: String::from("Who  is the current Colombian president"),
        options: options,
        answer: "a".to_string(),
        prize:10,
    };

    let question_b = Question {
        text: "What is Rust?".to_string(),
        options: vec![
            "a. it's Russian dish".to_string(),
            "b. it's a programming language".to_string(),
            "c. it's a block chain".to_string(),
            "d. it's a futbool soccer team".to_string(),
        ],
        answer: "b".to_string(),
        prize: 20,
    };


    let mut game =  Game{
        balance: 0,
        questions: vec![
            question_a,
            question_b,
        ]
    };

    game.run();

}
