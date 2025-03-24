# Étape 1 : Construction avec Composer
FROM composer:2 AS builder
WORKDIR /app
RUN composer create-project symfony/skeleton .
RUN composer require symfony/http-foundation symfony/routing symfony/http-kernel symfony/serializer --no-interaction

# Étape 2 : Image finale avec PHP-FPM
FROM php:8.2-fpm
WORKDIR /var/www
COPY --from=builder /app /var/www
# Copie explicite des fichiers locaux (Kernel.php et Controller)
COPY ./src /var/www/src
RUN apt-get update && apt-get install -y \
    libpq-dev \
    && docker-php-ext-install pdo pdo_mysql
EXPOSE 8082
CMD ["php-fpm"]