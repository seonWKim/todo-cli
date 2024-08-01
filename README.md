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
``` 

### Examples 
```bash
$ tc a "Buy milk"          # add a todo 
$ tc l                     # list all todos
$ tc f                     # find a todo
$ tc d <id>                # mark a todo as done 
$ tc undone <id>           # mark a todo as undone
$ tc i                     # start interactive mode
$ tc u <id> -t "Buy water" # update a todo 
$ tc t -m <minutes>        # set a timer 
````

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
- [ ] Timer support 
- [ ] Update todo 
