# TODO-CLI 

todo-cli is a simple command line tool to manage your todo list. 

## Features 

- [X] `tc -i`: Interactive mode 
- [X] `tc -a <task>`: Add a task to the list 
- [X] `tc -l`: List all tasks 
- [X] `tc -d <task>`: Mark a task as done
- [X] `tc -r <task>`: Remove a task from the list
- [X] `tc -h`: Show help 

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
# add a task  
$ tc a task1 

# list all tasks 
$ tc l 

# mark a task as done 
$ tc d task1 

# remove a task 
$ tc r task1 
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
- [ ] Confirm users when running `removeAll` command   
