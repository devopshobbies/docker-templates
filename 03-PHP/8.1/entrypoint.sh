#!/bin/sh

# echo  "Composer dump autoload"
# composer dump-autoload

# echo  "Run Migration command..."
# php artisan migrate --force

# echo  "Run Seeder command..."
# php artisan db:seed --force

# echo  "Run route clear command..."
# php artisan route:clear

# echo  "Run storage link..."
# php artisan storage:link --force

/usr/bin/supervisord  -c "/etc/supervisor/conf.d/supervisord.conf"
