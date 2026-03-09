#!/bin/bash

# Colegio CCB - Interactive Setup Script (Phase 26 Enhanced)
# Este script inicializa el sistema, configura el entorno y el primer usuario Root.

set -e

echo "==============================================="
echo "   🛡️  Colegio CCB - Setup Interactivo v2.0"
echo "==============================================="
echo ""

# 1. Verificar dependencias
if ! command -v docker > /dev/null 2>&1; then
    echo "❌ Error: Docker no está instalado."
    exit 1
fi

# 2. Información del Distribuidor
read -p "🏢 Nombre de la empresa distribuidora (Software Vendor): " DISTRIBUTOR_NAME

# 3. Limpieza de Base de Datos
read -p "🗑️  ¿Deseas limpiar la base de datos y empezar de cero? (y/N): " CLEAN_DB
if [[ $CLEAN_DB =~ ^[Yy]$ ]]; then
    echo "⚠️  Limpiando volúmenes de Docker..."
    docker compose down -v || true
fi

# 4. Modo de Despliegue
echo ""
echo "🌐 Selecciona el modo de despliegue:"
echo "1) Desarrollo (HTTP local)"
echo "2) Producción (HTTPS con SSL/Certbot)"
read -p "Opción [1-2]: " DEPLOY_OPTION

case $DEPLOY_OPTION in
    2)
        DEPLOY_MODE="production"
        NGINX_CONF="nginx/nginx.prod.conf"
        ;;
    *)
        DEPLOY_MODE="development"
        NGINX_CONF="nginx/nginx.dev.conf"
        ;;
esac

# 5. Configuración de .env
echo "📝 Configurando archivo .env..."
if [ ! -f .env ]; then
    cp .env.example .env || touch .env
fi

# Actualizar variables críticas
sed -i "s/^RUST_ENVIRONMENT=.*/RUST_ENVIRONMENT=$DEPLOY_MODE/" .env || echo "RUST_ENVIRONMENT=$DEPLOY_MODE" >> .env
sed -i "s/^NODE_ENV=.*/NODE_ENV=$DEPLOY_MODE/" .env || echo "NODE_ENV=$DEPLOY_MODE" >> .env

# Agregar DISTRIBUTOR_NAME si no existe
if grep -q "DISTRIBUTOR_NAME" .env; then
    sed -i "s/^DISTRIBUTOR_NAME=.*/DISTRIBUTOR_NAME=\"$DISTRIBUTOR_NAME\"/" .env
else
    echo "DISTRIBUTOR_NAME=\"$DISTRIBUTOR_NAME\"" >> .env
fi

# 6. Preparar Nginx
echo "⚙️  Configurando Nginx para modo $DEPLOY_MODE..."
cp "$NGINX_CONF" nginx/nginx.conf

# 7. Credenciales del Sistema
echo ""
echo "🔑 Configuración del Usuario ROOT (Plataforma):"
read -p "🏫 Nombre de la Instancia Principal: " SCHOOL_NAME
read -p "👤 Nombre del Administrador Root: " ADMIN_NAME
read -p "📧 Email del Administrador Root: " ADMIN_EMAIL
read -s -p "🔑 Contraseña del Administrador Root: " ADMIN_PASSWORD
echo ""
read -s -p "🔑 Confirmar Contraseña: " ADMIN_PASSWORD_CONFIRM
echo ""

if [ "$ADMIN_PASSWORD" != "$ADMIN_PASSWORD_CONFIRM" ]; then
    echo "❌ Error: Las contraseñas no coinciden."
    exit 1
fi

# 8. Iniciar Servicios
echo ""
echo "🚀 Iniciando contenedores..."
docker compose up -d --build

echo "⏳ Esperando a que la base de datos y el backend estén listos..."
sleep 10

# 9. Bootstrap (Creación de Rol Root y Usuario)
echo "🛠️  Ejecutando bootstrap del sistema..."
docker exec -i colleges_backend /app/bootstrap "$SCHOOL_NAME" "$ADMIN_NAME" "$ADMIN_EMAIL" "$ADMIN_PASSWORD"

# 10. SSL Initialization (solo en producción)
if [ "$DEPLOY_MODE" == "production" ]; then
    echo ""
    echo "🛡️  Iniciando proceso SSL con Certbot..."
    if [ -f "./init-ssl.sh" ]; then
        ./init-ssl.sh
    else
        echo "⚠️  Advertencia: init-ssl.sh no encontrado. No se generaron certificados."
    fi
fi

echo ""
echo "==============================================="
echo "   ✅ ¡Instalación completada con éxito!"
echo "   🏢 Distribuidor: $DISTRIBUTOR_NAME"
echo "   🌐 Modo: $DEPLOY_MODE"
echo "   📧 Usuario Root: $ADMIN_EMAIL"
echo "==============================================="
