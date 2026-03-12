#!/bin/bash

# ============================================
# Crear Usuario Administrador
# ============================================

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${GREEN}============================================${NC}"
echo -e "${GREEN}  Crear Usuario Administrador${NC}"
echo -e "${GREEN}============================================${NC}"
echo ""

# Cargar variables de entorno
if [ -f ".env" ]; then
    source .env
    echo -e "${GREEN}✓ Variables de entorno cargadas${NC}"
else
    echo -e "${RED}Error: No se encontró .env${NC}"
    exit 1
fi

DB_USER="${DB_USERNAME:-admin}"
DB_PASSWORD="${DB_PASSWORD:-password}"
DB_NAME="${DB_NAME:-colegios_main}"

echo -e "${YELLOW}DB: ${DB_NAME}, Usuario: ${DB_USER}${NC}"
echo ""

# Pedir datos del administrador
read -p "👤 Nombre del administrador: " ADMIN_NAME
read -p "📧 Email: " ADMIN_EMAIL

# Validar email
if [[ ! "$ADMIN_EMAIL" =~ ^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$ ]]; then
    echo -e "${RED}Email inválido${NC}"
    exit 1
fi

echo ""
read -s -p "🔑 Contraseña: " ADMIN_PASSWORD
echo ""
read -s -p "🔑 Confirmar contraseña: " ADMIN_PASSWORD_CONFIRM
echo ""

if [ "$ADMIN_PASSWORD" != "$ADMIN_PASSWORD_CONFIRM" ]; then
    echo -e "${RED}Las contraseñas no coinciden${NC}"
    exit 1
fi

if [ ${#ADMIN_PASSWORD} -lt 8 ]; then
    echo -e "${RED}La contraseña debe tener al menos 8 caracteres${NC}"
    exit 1
fi

# Obtener school_id
SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM schools WHERE subdomain = 'ccb' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

if [ -z "$SCHOOL_ID" ]; then
    SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
        "SELECT id FROM schools LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)
fi

# Obtener role_id para admin
ROLE_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM roles WHERE name = 'admin' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

if [ -z "$ROLE_ID" ]; then
    ROLE_ID=1
fi

echo -e "${YELLOW}School ID: ${SCHOOL_ID}, Role ID: ${ROLE_ID}${NC}"

if [ -z "$SCHOOL_ID" ] || [ -z "$ROLE_ID" ]; then
    echo -e "${RED}Error: No se pudo obtener información de la base de datos${NC}"
    exit 1
fi

# Eliminar usuario si existe
echo -e "${YELLOW}Limpiando usuarios existentes...${NC}"
docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
    "DELETE FROM users WHERE email = '${ADMIN_EMAIL}';" 2>/dev/null || true

# Generar hash con Python/argon2 o usar hash por defecto
echo -e "${YELLOW}Generando hash de contraseña...${NC}"

# Intentar usar argon2 si está disponible
if python3 -c "import argon2" 2>/dev/null; then
    PASSWORD_HASH=$(python3 -c "
import argon2
ph = argon2.PasswordHasher(time_cost=3, memory_cost=4096, parallelism=1)
print(ph.hash('${ADMIN_PASSWORD}'))
" 2>/dev/null)
fi

# Si no se pudo generar, usar hash por defecto
if [ -z "$PASSWORD_HASH" ]; then
    echo -e "${YELLOW}Usando hash Argon2 por defecto (contraseña: admin123)${NC}"
    echo -e "${RED}⚠️  IMPORTANTE: Cambia la contraseña después del primer login${NC}"
    PASSWORD_HASH='$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$i769B7jI77yXqV6N7z6w7w'
    ADMIN_PASSWORD="admin123"
fi

# Escapar caracteres especiales
PASSWORD_HASH_ESCAPED=$(echo "$PASSWORD_HASH" | sed "s/'/''/g")

# Crear usuario
echo -e "${YELLOW}Creando usuario: ${ADMIN_EMAIL}...${NC}"
RESULT=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c "
INSERT INTO users (school_id, role_id, name, email, password_hash, created_at, updated_at)
VALUES (
    '${SCHOOL_ID}',
    ${ROLE_ID},
    '${ADMIN_NAME}',
    '${ADMIN_EMAIL}',
    '${PASSWORD_HASH_ESCAPED}',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
RETURNING id;
" 2>&1)

if echo "$RESULT" | grep -q "INSERT"; then
    echo -e "${GREEN}✅ Usuario creado exitosamente${NC}"
    echo ""
    echo "============================================"
    echo -e "${GREEN}  ¡Éxito!${NC}"
    echo "============================================"
    echo ""
    echo "👤 Usuario: ${ADMIN_EMAIL}"
    echo "🌐 Frontend: http://localhost"
    echo "🔌 Backend: http://localhost:8080/health"
    echo ""
else
    echo -e "${RED}❌ Error al crear usuario${NC}"
    echo "Resultado: $RESULT"
    exit 1
fi
