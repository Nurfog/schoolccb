#!/bin/bash

# Script para crear usuario Root manualmente

DB_USER="user"
DB_PASSWORD="password"
DB_NAME="colleges"

echo "👑 Crear Usuario Root (Dueño de la Plataforma)"
echo "=============================================="
echo ""

read -p "👤 Nombre: " ADMIN_NAME
read -p "📧 Email: " ADMIN_EMAIL
read -s -p "🔑 Contraseña: " ADMIN_PASSWORD
echo ""

# Generar hash con Python
if command -v python3 &> /dev/null; then
    PASSWORD_HASH=$(python3 -c "
import argon2
ph = argon2.PasswordHasher(time_cost=3, memory_cost=4096, parallelism=1)
print(ph.hash('${ADMIN_PASSWORD}'))
" 2>/dev/null)
fi

if [ -z "$PASSWORD_HASH" ]; then
    echo "⚠️  Usando contraseña por defecto (admin123)"
    PASSWORD_HASH='$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$i769B7jI77yXqV6N7z6w7w'
    ADMIN_PASSWORD="admin123"
fi

# Obtener IDs
SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM schools WHERE subdomain = 'ccb' LIMIT 1;" | tr -d ' ')

ROLE_ID=1

# Eliminar usuario admin por defecto
docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
    "DELETE FROM users WHERE email = 'admin@ccb.edu.co';" 2>/dev/null

# Eliminar usuario si ya existe
docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
    "DELETE FROM users WHERE email = '${ADMIN_EMAIL}';" 2>/dev/null

# Crear usuario
echo ""
echo "📝 Creando usuario..."

docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c "
INSERT INTO users (school_id, role_id, name, email, password_hash)
VALUES ('${SCHOOL_ID}', ${ROLE_ID}, '${ADMIN_NAME}', '${ADMIN_EMAIL}', '${PASSWORD_HASH}');
"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ ¡Usuario creado exitosamente!"
    echo ""
    echo "🔐 Credenciales:"
    echo "   Email: ${ADMIN_EMAIL}"
    echo "   Contraseña: ${ADMIN_PASSWORD}"
    echo ""
    echo "🌐 Accede en: http://localhost"
else
    echo "❌ Error al crear el usuario"
fi
