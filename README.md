# Notebox - Rust CLI/TUI edition
A note-taking app for the terminal written in rust.

## Help
```
+-----------------=[N O T E B O X  A P P  H E L P]=-----------------+
|                      - Usage:  todo [args] -                      |
+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+
|--tui     |    | tui     | (or no args)  | open program tui        |
|--file    | -f | file    | [FILENAME]    | open file               |
|--add     | -a | add     | [TITLE]       | Add new note and quit   |
|--rm      | -r | rm      | [ID]          | Remove todo with ID     |
|--toggle  | -t | toggle  | [ID]          | Toggle todo with ID     |
|--list    | -l | ls      |               | list all todos          |
|--help    | -h | help    |               | get this help           |
|--version | -v | version |               | get version information |
|--batchrm | -br| batchrm | [MODE] [TODO] | Remove by data,more: -hb|
+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+
```
## And the help for batch remove:
```
+---------=[N O T E B O X  A P P  B A T C H  R E M O V E ]=---------+
|  -Usage: todo -br/--batchrm/batchrm [PATTERN]. Remove entrys by-  |
|           - a pattern. The patterns are listed below. -           |
+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+
| last     | l | remove last entry of the list of todos.            |
| first    | f | remove first entry of list of todos.               |
| ticked   | x | remove all ticked/finished entrys.                 |
| unticked | u | remove all unticked/unfinished entrys.             |
| dupes    | d | remove all duplicates, where one will remain.      |
| all      | * | remove all entrys.                                 |
+-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-+
```

## Installing/Building:
use
```
git clone https://github.com/SpielProfessor/Notebox-rust-cli
cd Notebox-rust-cli
cargo build
```
with cargo installed
