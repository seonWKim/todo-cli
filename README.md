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

### Add alias to your ~/.bashrc or ~/.zshrc 
```bash 
alias ta="tc a"
alias tl="tc l"
alias tla="tc l -a"
alias tf="tc f"
alias tfa="tc f -a"
alias ti="tc i"
alias td="tc d"
alias tu="tc u"
alias tt="tc t"
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
- [ ] Timer support 
- [ ] Update todo 
