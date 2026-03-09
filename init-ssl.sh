#!/bin/bash

# init-ssl.sh - Script para inicializar certificados SSL con Certbot y Nginx
# Basado en el enfoque de 'dummy certificates' para permitir que Nginx arranque.

if ! [ -x "$(command -v docker-compose)" ]; then
  echo 'Error: docker-compose no está instalado.' >&2
  exit 1
fi

# Cargar variables de entorno
if [ -f .env ]; then
  export $(grep -v '^#' .env | xargs)
fi

domains=($DOMAIN_NAME "www.$DOMAIN_NAME")
rsa_key_size=4096
data_path="./certbot"
email="$CERTBOT_EMAIL" # Agregado a .env
staging=0 # Cambiar a 1 para testear sin límites de Let's Encrypt

if [ -d "$data_path" ]; then
  read -p "Ya existen datos de Certbot. ¿Deseas borrarlos y empezar de nuevo? (y/N) " decision
  if [ "$decision" != "Y" ] && [ "$decision" != "y" ]; then
    exit
  fi
fi

if [ ! -e "$data_path/conf/options-ssl-nginx.conf" ] || [ ! -e "$data_path/conf/ssl-dhparams.pem" ]; then
  echo "### Descargando parámetros SSL recomendados..."
  mkdir -p "$data_path/conf"
  curl -s https://raw.githubusercontent.com/certbot/certbot/master/certbot-nginx/certbot_nginx/_internal/tls_configs/options-ssl-nginx.conf > "$data_path/conf/options-ssl-nginx.conf"
  curl -s https://raw.githubusercontent.com/certbot/certbot/master/certbot/certbot/ssl-dhparams.pem > "$data_path/conf/ssl-dhparams.pem"
fi

echo "### Creando certificados dummy para $domains..."
path="/etc/letsencrypt/live/$DOMAIN_NAME"
mkdir -p "$data_path/conf/live/$DOMAIN_NAME"
docker compose run --rm --entrypoint \
  "openssl req -x509 -nodes -newkey rsa:$rsa_key_size -days 1\
    -keyout '$path/privkey.pem' \
    -out '$path/fullchain.pem' \
    -subj '/CN=localhost'" certbot

echo "### Arrancando Nginx..."
docker compose up --force-recreate -d nginx

echo "### Borrando certificados dummy..."
docker compose run --rm --entrypoint \
  "rm -rf /etc/letsencrypt/live/$DOMAIN_NAME && \
   rm -rf /etc/letsencrypt/archive/$DOMAIN_NAME && \
   rm -rf /etc/letsencrypt/renewal/$DOMAIN_NAME.conf" certbot

echo "### Solicitando certificados reales a Let's Encrypt..."
domain_args=""
for domain in "${domains[@]}"; do
  domain_args="$domain_args -d $domain"
done

# Seleccionar email
case "$email" in
  "") email_arg="--register-unsafely-without-email" ;;
  *) email_arg="--email $email" ;;
esac

# Habilitar modo staging si es necesario
if [ $staging != "0" ]; then staging_arg="--staging"; fi

docker compose run --rm --entrypoint \
  "certbot certonly --webroot -w /var/www/certbot \
    $staging_arg \
    $email_arg \
    $domain_args \
    --rsa-key-size $rsa_key_size \
    --agree-tos \
    --force-renewal" certbot

echo "### Recargando Nginx..."
docker compose exec nginx nginx -s reload
