/*
-=[ N O T E B O X - R U S T  C L I  E D I T I O N ]=-
TODOs APP BY MrVollbart/SpielProfessor
Not yet done:
q/q!: difference
made in 2024
*/



// I M P O R T S //
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use todo::scan;
use todo::toint;
use serde::{Deserialize, Serialize};
use std::env;

// Main struct for entry
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    title:String,
    done:bool
}

// Create a new todos entry and return it
fn new_todo(title: String, done:bool) -> Todo {
    Todo {
        title,
        done
    }
}

// remove an entry
fn rm_todo(todos:&mut Vec<Todo>, id:usize) {
    if todos.len()>id {
        todos.remove(id);
    } else {
        println!("{}: no such ID", id);
    }
}

// toggle state of an entry
fn toggle_state(mut todos: Vec<Todo>, id:usize) -> Vec<Todo> {
    if todos.len()>id {
        todos[id].done = !todos[id].done;
    } else {
        println!("{}: no such ID", id);
    }
    todos
}

// Default file path
const DEFAULT_PATH: &'static str = "todos.ron";
// Enable debug save/load messages
const DEBUG:bool = false;

// main function
#[allow(unused_mut, unused_assignments)]
fn main() {
    // initialize variables
    let version="0.1b";
    let version_release_year=2024;
    let args:Vec<String> =env::args().collect();
    let mut todos = vec![new_todo(String::from("Hi"), false), new_todo("buy dogs".to_string(), true)];
    let mut running = true;
    let mut no_prev_path=true;
    let mut filepath = DEFAULT_PATH.to_string();

    // check if there are arguments
    if args.len()>1 {
        let mut c_arg =0;
        let mut should_jump=false;
        // loop through all args
        for arg in &args {
            if should_jump {
                should_jump=false;
                c_arg+=1;
                continue;
            }
            // switch to default path if there is no other path given
            if no_prev_path {
                todos=default_path( todos);
                filepath=DEFAULT_PATH.to_string();
            }


            // Enable TUI mode from arguments
            if arg.trim() == "--tui" || arg.trim() == "tui" {
                (todos, running) = tui(todos, running);


                // show help
            } else if arg.trim() == "-h" || arg.trim() == "--help" || arg.trim() == "help" {
                help();


                //batch remove help
            } else if arg.trim() == "-hb" {
                batchhelp();

                // Open file
            } else if arg.trim() == "-f" || arg.trim() == "--file" || arg.trim() == "file" {
                let pth = Path::new(&args[c_arg + 1]);
                if pth.exists() {
                    if DEBUG {println!("File {} exists, loading it...", &args[c_arg + 1]);}
                    todos = open(args[c_arg+1].to_string());
                    filepath = args[c_arg + 1].clone();
                    no_prev_path=false;
                } else {
                    println!("{}: No such file", &args[c_arg +1 ]);
                    running=false;
                }
                should_jump=true;


                // list all todos
            } else if arg.trim()=="-l" || arg.trim() == "--list" || arg.trim() =="ls" {
                println!("-=[Entries]=-");
                list_todos(&mut todos);


                // add new entry
            } else if arg.trim()=="-a" || arg.trim() == "--add" || arg.trim() == "add" {
                todos.push(new_todo(args[c_arg + 1].clone(), false));
                if DEBUG {list_todos(&mut todos);}
                save(filepath.clone(), &mut todos);
                should_jump=true;


                // remove entry
            } else if arg.trim()=="-r" || arg.trim() == "--rm" || arg.trim() == "rm" {
                rm_todo(&mut todos, toint(args[c_arg + 1].clone()) as usize);
                save(filepath.clone(), &mut todos);
                should_jump=true;


                // toggle entry
            } else if arg.trim() == "-t" || arg.trim() == "--toggle" || arg.trim() == "toggle" {
                todos=toggle_state(todos, toint(args[c_arg + 1].clone()) as usize);
                save(filepath.clone(), &mut todos);
                should_jump=true;


                // get current version
            } else if arg.trim() == "-v" || arg.trim() == "--version" || arg.trim() == "version" {
                println!("Todo v. {} by MrVollbart/spielprofessor, released in {}",version,version_release_year);
                running=false;


                // batch removal of todos
            } else if arg.trim() == "-br" || arg.trim() == "--batchrm" || arg.trim() == "batchrm" {
                todos = batchrm(todos, args[c_arg + 1].clone().trim());
                save(filepath.clone(), &mut todos);
                should_jump=true;
            }
            c_arg +=1;
        }
    }


    // default mode if no args given
    if args.len()<=1 {
        if no_prev_path {
            todos=default_path( todos);
            filepath=DEFAULT_PATH.to_string();
        }
        (todos, running) = tui(todos, running);
    }
}


// set path to default path and open it
fn default_path(mut todos: Vec<Todo>) -> Vec<Todo>{
    let path = Path::new(DEFAULT_PATH);
    if path.exists() {
        if DEBUG {println!("File {} exists, loading it...", DEFAULT_PATH);}
        todos = open(DEFAULT_PATH.to_string());
    }
    todos
}


// get program help
fn help(){
    println!("+----=[N O T E B O X  A P P  B Y  M R V O L L B A R T  H E L P]=----+");
    println!("|                      - Usage:  todo [args] -                      |");
    println!("+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+");
    println!("|--tui     |    | tui     | (or no args)  | open program tui        |");
    println!("|--file    | -f | file    | [FILENAME]    | open file               |");
    println!("|--add     | -a | add     | [TITLE]       | Add new note and quit   |");
    println!("|--rm      | -r | rm      | [ID]          | Remove todo with ID     |");
    println!("|--toggle  | -t | toggle  | [ID]          | Toggle todo with ID     |");
    println!("|--list    | -l | ls      |               | list all todos          |");
    println!("|--help    | -h | help    |               | get this help           |");
    println!("|--version | -v | version |               | get version information |");
    println!("|--batchrm | -br| batchrm | [MODE] [TODO] | Remove by data,more: -hb|");
    println!("+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+");
}
fn batchhelp(){
    println!("+---------=[N O T E B O X  A P P  B A T C H  R E M O V E ]=---------+");
    println!("|  -Usage: todo -br/--batchrm/batchrm [PATTERN]. Remove entrys by-  |");
    println!("|           - a pattern. The patterns are listed below. -           |");
    println!("+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+");
    println!("| last     | l | remove last entry of the list of todos.            |");
    println!("| first    | f | remove first entry of list of todos.               |");
    println!("| ticked   | x | remove all ticked/finished entrys.                 |");
    println!("| unticked | u | remove all unticked/unfinished entrys.             |");
    println!("| dupes    | d | remove all duplicates, where one will remain.      |");
    println!("| all      | * | remove all entrys.                                 |");
    println!("+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+");
}

fn batchrm(mut todos: Vec<Todo>, arg: &str) -> Vec<Todo> {
    match arg {
        "last" | "l" => _ = todos.remove(todos.len()-1),
        "first" | "f" => _ = todos.remove(0),
        "ticked" | "x" => {
            let mut index = todos.len();
            while index > 0 {
                index -= 1;
                if todos[index].done {
                    todos.remove(index);
                }
            }
        },
        "unticked" | "u" => {
            let mut index = todos.len();
            while index > 0 {
                index -= 1;
                if !todos[index].done {
                    todos.remove(index);
                }
            }
        },
        "dupes" | "d" => {
            //check for duplicates
            let mut index = todos.len();
            //every value is inserted here
            let mut previous:Vec<Todo> = Vec::new();
            while index > 0 {
                index-=1;
                let mut broken=false;
                let mut prev_index=previous.len();
                // check if already in encountered values: if yes, remove it
                while prev_index>0 {
                    prev_index-=1;
                    if todos[index].title == previous[prev_index].title {
                        todos.remove(index);
                        broken=true;
                        break;
                    }
                }
                if !broken {
                    previous.push(todos[index].clone());
                }

            }
        },
        "all" | "*" => todos.clear(),
        _ => batchhelp()
    }
    todos
}


// open the TUI
fn tui(mut todos: Vec<Todo>, mut running: bool) -> (Vec<Todo>, bool){
    // main loop
    while running {
        println!("+----[ T O D O S ]----+");
        println!("|   A rust todo app   |");
        println!("+-=-=-=-=-=-=-=-=-=-=-+");
        list_todos(&mut todos);
        println!("-=-=-=-=-=-=-=-=-=-=-=-");
        println!("What do you want to do?");
        println!("a: create new todo, d: remove todo, t: tick/untick todo, q/q!/wq: quit, w: write, o:open");
        print!(":");
        // get user input for functions
        let user_input = scan().trim().to_string();
        match user_input[..1].to_string().as_str(){


            // add todos entry
            "a" => {
                    if user_input.len()>1 {
                        todos.push(new_todo(user_input[2..].to_string(), false));
                    } else {
                        print!("Content of todo:\n:");
                        todos.push(new_todo(scan().trim().to_string(), false));
                    }
                 }


            //delete todos entry
            "d" => {
                    if user_input.len()>1 {
                        rm_todo(&mut todos, toint(user_input[2..].to_string()) as usize);
                    } else {
                        print!("Please insert id (first number in list)\n:");
                        rm_todo(&mut todos, toint(scan()) as usize)
                    }
                 }


            //toggle entry
            "t" => {
                    if user_input.len()>1 {
                        todos= toggle_state(todos, toint(user_input[2..].to_string()) as usize);
                    } else {
                        print!("Please insert id (first number in list)\n:");
                        todos = toggle_state(todos, toint(scan()) as usize);
                    }
                 },


            //write
            "w" =>  {
                        // if user inserts more arguments, check for wq and quit if true
                        if user_input.len()>1 && user_input[1..2].to_string().as_str()=="q" {
                            running = false;
                        }
                        if user_input.len()>2 {
                            save(user_input[2..].to_string(), &mut todos);
                        } else {
                            save(DEFAULT_PATH.to_string(), &mut todos);
                        }
                     },
            // quit
            "q" => {
                        running=false
                   },

            //open file
            "o" => {
                        #[allow(unused_assignments)]
                        let mut path=Path::new("");
                        #[allow(unused_assignments)]
                        let mut path_text=String::new();
                        if user_input.len()>1 {
                            path_text=user_input[2..].to_string();
                            path=Path::new(&path_text);
                        } else {
                            path_text = scan().trim_end().to_string();
                            path = Path::new(&path_text);
                        }
                            if path.exists() && path.is_file() {
                                if DEBUG {println!("Path exists! continuing...");}
                                todos=open(path_text);
                            } else {
                                println!("Path {} doesn't seem to exist!", path_text);
                            }
                   },

            // wrong/unknown option
            _ => println!("That option is not available")
        }
    }
    println!("Thanks for using my todo app!");
    (todos, running)
}



// list all todos
fn list_todos(todos: &mut Vec<Todo>) {
    for i in 0..todos.len() {
        print!("{}: ", i);
        if todos[i].done == true {
            print!("[x] ");
        } else {
            print!("[ ] ");
        }
        println!("{}", todos[i].title);
    }
}


// save todos to path
fn save(path: String, todos: &mut Vec<Todo>){
    println!("writing to {}", path);
    let serialized:String = ron::ser::to_string(todos).unwrap().to_string();
    let file = File::create(path);
    file.unwrap().write_all(serialized.as_ref()).unwrap();
}

// open todos from path
fn open(path: String) -> Vec<Todo> {
    if DEBUG {println!("Opening {}", path);}
    let file=File::open(path.clone());
    let mut contents=String::new();
    file.unwrap().read_to_string(&mut contents).expect(format!("File opening error! Does the file {} exist?", path).as_str());
    let todos:Vec<Todo> = ron::from_str(&contents as &str).expect("There seemed to be an error");
    todos
}
