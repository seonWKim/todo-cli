# TODO-CLI 

todo-cli is a simple command line tool to manage your todo list. 

## Installation 

```bash
# clone the repository and change directory  
$ git clone https://github.com/seonWKim/todo-cli.git 
$ cd todo-cli 

# build the project 
$ ./release.sh
```
                    
## Usage 

```bash
# start interactive mode   
$ tc i 

# get all commands 
$ tc h 
``` 

## Development 

### Debugging sqlite  
```bash
# connect to the database 
$ sqlite3 ~/.tc/todo.db

# list tables 
sqlite> .tables 

# select all todos(table)  
sqlite> SELECT * FROM todos;  
```

### TODO 
- [ ] Add priority to the tasks, Sort and show by priority  
- [ ] Confirm users when running `reset` command   
- [ ] Add Alfred support 
