# PHP-Laravel

## get started

- copy `docker/`, `.htaccess`, `Dockerfile`, `.dockerignore` into laravel root project.

- run command `docker build -t laravel-project .` to build image

- run command `docker run -p 8000:80 --name laravel_project -d laravel-project` to run container 
## used tools

- php 8.0
- nginx
- supervisor
- mysql
- composer


## configurations

- please update `docker/.env.production` by your own settings

