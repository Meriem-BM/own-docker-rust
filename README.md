[![progress-banner](https://backend.codecrafters.io/progress/docker/f568baf8-f7a8-4b17-824b-02e98cf86802)](https://app.codecrafters.io/users/Meriem-BM?r=2qF)
# 1st stage

The entry point for your Docker implementation is `src/main.rs`. Study and
uncomment the relevant code, and push your changes to pass the first stage:

```sh
git add .
git commit -m "pass 1st stage" # any msg
git push origin master
```

That's all!

# 2nd stage

Note: This section is for stages 2 and beyond.

You'll use linux-specific syscalls in this challenge. so we'll run your code
_inside_ a Docker container.

Please ensure you have [Docker installed](https://docs.docker.com/get-docker/)
locally.

Next, add a [shell alias](https://shapeshed.com/unix-alias/):

```sh
alias mydocker='docker build -t mydocker . && docker run --cap-add="SYS_ADMIN" mydocker'
```

(The `--cap-add="SYS_ADMIN"` flag is required to create
[PID Namespaces](https://man7.org/linux/man-pages/man7/pid_namespaces.7.html))

You can now execute your program like this:

```sh
mydocker run ubuntu:latest /usr/local/bin/docker-explorer echo hey
```

This command compiles your Rust project, so it might be slow the first time you
run it. Subsequent runs will be fast.
