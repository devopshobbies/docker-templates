#!/bin/sh

cd /var/www

php artisan key:generate
php artisan migrate
# php artisan migrate:fresh --seed
php artisan passport:install
php artisan cache:clear
# Optimizing Configuration loading
php artisan config:cache
# Optimizing Route loading
php artisan route:cache
# Optimizing View loading
php artisan view:cache

/usr/bin/supervisord -c /etc/supervisord.conf
# /usr/sbin/nginx -g 'daemon off;'