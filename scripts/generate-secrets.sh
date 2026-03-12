#!/bin/bash
# ============================================
# Script para Generar Secretos Seguros
# ============================================
# Uso: ./scripts/generate-secrets.sh
# ============================================

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}============================================${NC}"
echo -e "${GREEN}  Generador de Secretos Seguros${NC}"
echo -e "${GREEN}  Sistema de Administración de Colegios${NC}"
echo -e "${GREEN}============================================${NC}"
echo ""

# Función para generar secreto aleatorio
generate_secret() {
    local length=$1
    openssl rand -base64 $length | tr -d '\n'
}

# Verificar si openssl está disponible
if ! command -v openssl &> /dev/null; then
    echo -e "${RED}Error: openssl no está instalado${NC}"
    echo "Instalar con: sudo apt install openssl"
    exit 1
fi

echo -e "${YELLOW}Generando secretos...${NC}"
echo ""

# Generar secretos
JWT_SECRET=$(generate_secret 64)
SESSION_SECRET=$(generate_secret 64)
DB_PASSWORD=$(generate_secret 48)

echo -e "${GREEN}✅ Secretos generados exitosamente${NC}"
echo ""
echo "============================================"
echo "COPIA ESTOS VALORES EN TU .env:"
echo "============================================"
echo ""
echo "# JWT Secret (64 caracteres)"
echo "JWT_SECRET_KEY=${JWT_SECRET}"
echo ""
echo "# Session Secret (64 caracteres)"
echo "SESSION_SECRET=${SESSION_SECRET}"
echo ""
echo "# Database Password (32 caracteres)"
echo "DB_PASSWORD=${DB_PASSWORD}"
echo ""
echo "============================================"
echo ""

# Preguntar si quiere actualizar el .env automáticamente
read -p "¿Actualizar archivo .env automáticamente? (s/n): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Ss]$ ]]; then
    if [ -f ".env" ]; then
        # Crear backup del .env actual
        cp .env .env.backup.$(date +%Y%m%d_%H%M%S)
        echo -e "${YELLOW}✓ Backup creado: .env.backup.$(date +%Y%m%d_%H%M%S)${NC}"
        
        # Actualizar valores en .env
        sed -i "s/^JWT_SECRET_KEY=.*/JWT_SECRET_KEY=${JWT_SECRET}/" .env
        sed -i "s/^SESSION_SECRET=.*/SESSION_SECRET=${SESSION_SECRET}/" .env
        sed -i "s/^DB_PASSWORD=.*/DB_PASSWORD=${DB_PASSWORD}/" .env
        
        # Actualizar DATABASE_URL con la nueva contraseña
        DB_USERNAME=$(grep "^DB_USERNAME=" .env | cut -d '=' -f 2)
        DB_NAME=$(grep "^DB_NAME=" .env | cut -d '=' -f 2)
        sed -i "s|^DATABASE_URL=.*|DATABASE_URL=postgres://${DB_USERNAME}:${DB_PASSWORD}@db:5432/${DB_NAME}|" .env
        
        echo -e "${GREEN}✓ .env actualizado exitosamente${NC}"
        echo -e "${YELLOW}⚠️  Reinicia los contenedores para aplicar los cambios:${NC}"
        echo "   docker compose down && docker compose up -d"
    else
        echo -e "${RED}Error: No se encontró el archivo .env${NC}"
    fi
fi

echo ""
echo -e "${GREEN}============================================${NC}"
echo -e "${GREEN}  Proceso completado${NC}"
echo -e "${GREEN}============================================${NC}"
