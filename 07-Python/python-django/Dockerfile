FROM python:3.10-slim-buster

WORKDIR /app
ENV DJANGO_SETTINGS_MODULE=sample_backend.settings.production
ENV DATABASE_HOST='mysql'

RUN pip install -U pip
# RUN apt-get update
# RUN apt-get install -y python3-pip cron nano build-essential python3-dev default-libmysqlclient-dev  default-mysql-client  ffmpeg
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
   --mount=target=/var/cache/apt,type=cache,sharing=locked \
   rm -f /etc/apt/apt.conf.d/docker-clean \
   && apt-get update \
   && apt-get -y --no-install-recommends install \
   python3-pip cron nano build-essential python3-dev default-libmysqlclient-dev  default-mysql-client  ffmpeg libmagic1


COPY ./requirements.txt /app
RUN pip3 install -r requirements.txt

COPY ./ .

COPY docker_entrypoint.sh app.sh

# Run server
EXPOSE 8000

CMD uwsgi --http 0.0.0.0:8000 \
   --thunder-lock \
   --single-interpreter \
   --enable-threads \
   --processes=${UWSGI_WORKERS:-2} \
   --buffer-size=8192 \
   # --chdir /app/ \
   # --wsgi-file talent.wsgi.py \
   --module sample_backend.wsgi --py-autoreload 1 
