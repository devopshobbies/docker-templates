
# Python3 - Django

## used tools

- mysql
- python 3.10
- uwsgi

## get started

1. change `DJANGO_SETTINGS_MODULE` environment based on your project

2. create a `requirements.txt` file and append follow packages:
```
uwsgi==2.0.20 
```

3. you can edit `docker_entrypoint.sh` file, based on your project configurations.

4. change `sample_backend.wsgi` on last command on `Dockerfile` based on your project