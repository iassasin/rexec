# Rexec

Simple http service for remote task execution

### Run setup
1. Copy `rexec.example.yml` to `rexec.yml`
2. `cargo run`
3. `curl -v localhost:8080/task/test`
4. See `Hello, world` in terminal, executed remotely output
5. `curl -v localhost:8080/task/echo -d 'hi!'`
6. See `hi!` in terminal