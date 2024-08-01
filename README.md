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
                
### Basic 
```bash
# start interactive mode   
$ tc i 

# get all commands 
$ tc h 

# add a todo 
$ tc a "Buy milk"

# list all todos
$ tc l 

# find a todo
$ tc f

# mark a todo as done
$ tc d <id>

# mark a todo as undone
$ tc undone <id>

# update a todo
$ tc u <id> -t "Buy water"

# set a timer
$ tc t -m <minutes>      
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
- [ ] Add priority to the todos, sort and show by priority
- [ ] Add Alfred support 
- [ ] Deadline for todos 
