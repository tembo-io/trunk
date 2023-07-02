
Building with Trunk.toml should be the same as:
```
trunk build --dockerfile ./Dockerfile --install-command "cd pg_cron && make install" --name pg_cron --version 1.5.2
```
