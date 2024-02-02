**CLI Todo app.**
> Check releases for the executable binary

```shell
Usage: todo <COMMAND>

Commands:
  add     Add new task
  delete  Delete task with id
  update  Update task with id
  done    Toggle task done status
  list    List all tasks
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# ADD

## Add new todo item.

```shell
Add new task

Usage: todo add --summary <SUMMARY>

Options:
  -s, --summary <SUMMARY>  Task summary
  -h, --help               Print help
```

# DELETE

## Delete todo item.

```shell
Delete task with id

Usage: todo delete --id <ID>

Options:
  -i, --id <ID>  Task id to delete
  -h, --help     Print help
```

# UPDATE

## Update todo item.

```shell
Update task with id

Usage: todo update --summary <SUMMARY> --id <ID>

Options:
  -s, --summary <SUMMARY>  New summary
  -i, --id <ID>            Task id to update
  -h, --help               Print help
```

# SET/TOGGLE DONE

## Mark task done or toggle done status.

```shell
Mark task done

Usage: todo done [OPTIONS] --id <ID>

Options:
  -i, --id <ID>  Task id to mark done
  -t, --toggle   Toggles if already done (default `false`)
  -h, --help     Print help
```

# LIST

## List todo items.

```shell
List tasks

Usage: todo list [OPTIONS]

Options:
  -a, --all   Show done tasks as well (default `false`)
  -h, --help  Print help
```

# OUTPUT

<img width="916" alt="todo_rs_demo" src="https://github.com/kenalizadeh/todo_rs/assets/4370392/894852b8-225d-42ba-8b89-022188df3617">

