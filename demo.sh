rm -rf ~/todo.db && \
  set -x && \
  cargo build --release && \
  todo add -s "Write tests for todo_rs" && \
  todo add -s "Beat Sword Saint Isshin without Kuro's charm" && \
  todo update -i 1 -s "Write tests for todo_rs at least in the foreseeable future maybe?" && \
  todo done -i 1 && \
  todo done -i 1 -t && \
  todo done -i 2 && \
  todo delete -i 1 && \
  todo list && \
  todo list -a
