#!/bin/bash

# Colegio CCB - Interactive Setup Script
# Este script inicializa el sistema y crea el primer SuperAdmin.

set -e

echo "==============================================="
echo "   🛡️  Colegio CCB - Setup Interactivo"
echo "==============================================="
echo ""

# Verificar si Docker está corriendo
if ! docker info > /dev/null 2>&1; then
    echo "❌ Error: Docker no está corriendo. Por favor inicia Docker y vuelve a intentarlo."
    exit 1
fi

# Solicitar información al usuario
read -p "🏫 Nombre del Colegio (ej. Colegio Central Bogotá): " SCHOOL_NAME
read -p "👤 Nombre del Super Administrador: " ADMIN_NAME
read -p "📧 Email del Super Administrador: " ADMIN_EMAIL
read -s -p "🔑 Contraseña del Super Administrador: " ADMIN_PASSWORD
echo ""
read -s -p "🔑 Confirmar Contraseña: " ADMIN_PASSWORD_CONFIRM
echo ""

if [ "$ADMIN_PASSWORD" != "$ADMIN_PASSWORD_CONFIRM" ]; then
    echo "❌ Error: Las contraseñas no coinciden."
    exit 1
fi

echo ""
echo "🚀 Iniciando contenedores y base de datos..."
docker compose up -d

echo "⏳ Esperando a que el backend esté listo..."
sleep 5

echo "🛠️  Ejecutando bootstrap..."
# Ejecutamos el comando bootstrap dentro del contenedor del backend
docker exec -it colleges_backend /app/bootstrap "$SCHOOL_NAME" "$ADMIN_NAME" "$ADMIN_EMAIL" "$ADMIN_PASSWORD"

echo ""
echo "==============================================="
echo "   ✅ ¡Instalación completada con éxito!"
echo "   🌐 Accede a: http://localhost"
echo "   📧 Usuario: $ADMIN_EMAIL"
echo "==============================================="
