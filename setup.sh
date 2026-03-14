#!/bin/bash

# ============================================
# SchoolCCB SaaS - Interactive Setup Script
# ============================================
# Versión: 1.0.0 (Marzo 2026)
# Setup automatizado para desarrollo y producción
# ============================================
# Estado: 100% Completado ✅
# Features: IA/ML, 2FA, Redis, Email Queue, PDFs
# ============================================

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m' # No Color

# ============================================
# Funciones Auxiliares
# ============================================

print_header() {
    echo -e "${CYAN}"
    echo "==============================================="
    echo "   🎉 SchoolCCB SaaS - Setup v1.0.0"
    echo "   100% Completado - 89+ Endpoints"
    echo "==============================================="
    echo -e "${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ Error: $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_feature() {
    echo -e "${MAGENTA}🚀 $1${NC}"
}

wait_for_service() {
    local service=$1
    local max_attempts=${2:-30}
    local attempt=1

    print_info "Esperando a que $service esté listo..."

    while [ $attempt -le $max_attempts ]; do
        if docker compose ps | grep -q "$service.*healthy"; then
            print_success "$service está listo"
            return 0
        fi
        sleep 2
        attempt=$((attempt + 1))
        print_info "Intento $attempt/$max_attempts - esperando $service..."
    done

    print_error "Timeout esperando $service después de $max_attempts intentos"
    docker compose logs "$service" 2>/dev/null || true
    return 1
}

generate_secure_secret() {
    local length=${1:-64}
    openssl rand -base64 $length 2>/dev/null | tr -d '\n' || \
    python3 -c "import secrets; print(secrets.token_urlsafe($length))" 2>/dev/null || \
    echo "CHANGE_THIS_SECRET_KEY_IN_PRODUCTION"
}

validate_password_strength() {
    local password="$1"
    
    # Verificar longitud mínima (8 caracteres)
    if [ ${#password} -lt 8 ]; then
        print_error "La contraseña debe tener al menos 8 caracteres"
        return 1
    fi
    
    # Verificar mayúscula
    if ! [[ "$password" =~ [A-Z] ]]; then
        print_error "La contraseña debe tener al menos una mayúscula"
        return 1
    fi
    
    # Verificar minúscula
    if ! [[ "$password" =~ [a-z] ]]; then
        print_error "La contraseña debe tener al menos una minúscula"
        return 1
    fi
    
    # Verificar número
    if ! [[ "$password" =~ [0-9] ]]; then
        print_error "La contraseña debe tener al menos un número"
        return 1
    fi
    
    # Verificar carácter especial
    if ! [[ "$password" =~ [^A-Za-z0-9] ]]; then
        print_error "La contraseña debe tener al menos un carácter especial"
        return 1
    fi
    
    return 0
}

# ============================================
# Script Principal
# ============================================

print_header

# 1. Verificar dependencias
print_info "Verificando dependencias..."
if ! command -v docker &> /dev/null; then
    print_error "Docker no está instalado"
    exit 1
fi

if ! docker compose version &> /dev/null; then
    print_error "Docker Compose no está instalado"
    exit 1
fi

print_success "Docker y Docker Compose están instalados"

# 2. Información del Distribuidor
echo ""
read -p "🏢 Nombre de la empresa distribuidora: " DISTRIBUTOR_NAME

# 3. Limpieza de Base de Datos
echo ""
read -p "🗑️  ¿Deseas limpiar la base de datos y empezar de cero? (y/N): " CLEAN_DB
if [[ $CLEAN_DB =~ ^[Yy]$ ]]; then
    print_warning "Limpiando volúmenes de Docker..."
    docker compose down -v 2>/dev/null || true
    print_success "Base de datos limpiada"
fi

# 4. Modo de Despliegue
echo ""
echo "🌐 Selecciona el modo de despliegue:"
echo "  1) Desarrollo (HTTP, logs detallados)"
echo "  2) Producción (HTTPS con SSL, optimizado)"
read -p "Opción [1-2]: " DEPLOY_OPTION

case $DEPLOY_OPTION in
    2)
        DEPLOY_MODE="production"
        print_info "Modo producción seleccionado"
        ;;
    *)
        DEPLOY_MODE="development"
        print_info "Modo desarrollo seleccionado"
        ;;
esac

# 5. Configuración de Redis (Opcional para Email Queue)
echo ""
echo "📧 Configuración de Redis para Email Queue:"
read -p "¿Deseas habilitar Redis para colas de email? (y/N): " ENABLE_REDIS
if [[ $ENABLE_REDIS =~ ^[Yy]$ ]]; then
    REDIS_PROFILE="--profile with-redis"
    print_feature "Redis habilitado para colas de email asíncrono"
else
    REDIS_PROFILE=""
    print_info "Redis no habilitado (emails se enviarán síncronamente)"
fi

# 6. Configuración de IA/ML (Opcional)
echo ""
echo "🤖 Configuración de IA/ML (Ollama + Whisper):"
read -p "¿Deseas habilitar integración con IA? (y/N): " ENABLE_AI
if [[ $ENABLE_AI =~ ^[Yy]$ ]]; then
    print_info "Configuración de IA:"
    read -p "URL de Ollama (default: http://t-800.norteamericano.cl:11434): " OLLAMA_URL
    OLLAMA_URL=${OLLAMA_URL:-http://t-800.norteamericano.cl:11434}
    
    read -p "URL de Whisper (default: http://t-800.norteamericano.cl:9000): " WHISPER_URL
    WHISPER_URL=${WHISPER_URL:-http://t-800.norteamericano.cl:9000}
    
    read -p "Modelo para chat (default: llama3.2): " OLLAMA_MODEL
    OLLAMA_MODEL=${OLLAMA_MODEL:-llama3.2}
    
    print_feature "IA/ML habilitada con 8 endpoints"
else
    OLLAMA_URL=""
    WHISPER_URL=""
    OLLAMA_MODEL=""
    print_info "IA/ML no habilitada (puedes configurarla después en .env)"
fi

# 7. Configuración de .env
echo ""
print_info "Configurando archivo .env..."

if [ ! -f .env ]; then
    cp .env.example .env
    print_success "Archivo .env creado desde .env.example"
fi

# Generar secretos seguros
print_info "Generando secretos seguros..."

# JWT_SECRET_KEY (mínimo 32 caracteres)
if ! grep -q "^JWT_SECRET_KEY=" .env || grep -q "^JWT_SECRET_KEY=$" .env || grep -q "changeme" .env; then
    JWT_SECRET=$(generate_secure_secret 64)
    sed -i "s|^JWT_SECRET_KEY=.*|JWT_SECRET_KEY=${JWT_SECRET}|" .env 2>/dev/null || \
    echo "JWT_SECRET_KEY=${JWT_SECRET}" >> .env
    print_success "JWT_SECRET_KEY generada (64 caracteres)"
fi

# SESSION_SECRET
if ! grep -q "^SESSION_SECRET=" .env || grep -q "^SESSION_SECRET=$" .env; then
    SESSION_SECRET=$(generate_secure_secret 64)
    sed -i "s|^SESSION_SECRET=.*|SESSION_SECRET=${SESSION_SECRET}|" .env 2>/dev/null || \
    echo "SESSION_SECRET=${SESSION_SECRET}" >> .env
    print_success "SESSION_SECRET generada"
fi

# DB_PASSWORD (si es default)
if grep -q "changeme123!" .env; then
    DB_PASSWORD=$(generate_secure_secret 48)
    sed -i "s|^DB_PASSWORD=.*|DB_PASSWORD=${DB_PASSWORD}|" .env 2>/dev/null || \
    echo "DB_PASSWORD=${DB_PASSWORD}" >> .env
    print_success "DB_PASSWORD generada"
fi

# Actualizar variables de entorno específicas
sed -i "s|^RUST_ENVIRONMENT=.*|RUST_ENVIRONMENT=${DEPLOY_MODE}|" .env 2>/dev/null || true
sed -i "s|^NODE_ENV=.*|NODE_ENV=${DEPLOY_MODE}|" .env 2>/dev/null || true
sed -i "s|^DISTRIBUTOR_NAME=.*|DISTRIBUTOR_NAME=\"${DISTRIBUTOR_NAME}\"|" .env 2>/dev/null || true

# Configurar IA si fue habilitada
if [ -n "$OLLAMA_URL" ]; then
    sed -i "s|^OLLAMA_URL=.*|OLLAMA_URL=${OLLAMA_URL}|" .env 2>/dev/null || true
    sed -i "s|^WHISPER_URL=.*|WHISPER_URL=${WHISPER_URL}|" .env 2>/dev/null || true
    sed -i "s|^OLLAMA_MODEL=.*|OLLAMA_MODEL=${OLLAMA_MODEL}|" .env 2>/dev/null || true
fi

# Configurar Redis
if [ -n "$REDIS_PROFILE" ]; then
    sed -i "s|^REDIS_URL=.*|REDIS_URL=redis://redis:6379|" .env 2>/dev/null || true
fi

print_success "Archivo .env configurado"

# 8. Credenciales del Dueño de la Plataforma (Root)
echo ""
echo "👑 Configuración del Dueño de la Plataforma (Root):"
echo "---------------------------------------------------"
echo "Este usuario tendrá acceso a la Consola Root para gestionar"
echo "TODOS los colegios, licencias y métricas de la plataforma SaaS."
echo ""
read -p "👤 Nombre del Administrador: " ADMIN_NAME
read -p "📧 Email del Administrador: " ADMIN_EMAIL

# Validar email
if [[ ! "$ADMIN_EMAIL" =~ ^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$ ]]; then
    print_error "Email inválido"
    exit 1
fi

echo ""
echo "🔐 Requisitos de contraseña:"
echo "  - Mínimo 8 caracteres"
echo "  - Al menos 1 mayúscula"
echo "  - Al menos 1 minúscula"
echo "  - Al menos 1 número"
echo "  - Al menos 1 carácter especial"
echo ""

while true; do
    read -s -p "🔑 Contraseña: " ADMIN_PASSWORD
    echo ""
    read -s -p "🔑 Confirmar Contraseña: " ADMIN_PASSWORD_CONFIRM
    echo ""
    
    if [ "$ADMIN_PASSWORD" != "$ADMIN_PASSWORD_CONFIRM" ]; then
        print_error "Las contraseñas no coinciden"
        continue
    fi
    
    if validate_password_strength "$ADMIN_PASSWORD"; then
        break
    fi
done

# 9. Iniciar Servicios
echo ""
print_info "Iniciando servicios Docker..."

if [ -n "$REDIS_PROFILE" ]; then
    if ! docker compose $REDIS_PROFILE up -d --build; then
        print_error "Failed to start Docker services"
        docker compose logs 2>/dev/null || true
        exit 1
    fi
else
    if ! docker compose up -d --build; then
        print_error "Failed to start Docker services"
        docker compose logs 2>/dev/null || true
        exit 1
    fi
fi

print_success "Servicios Docker iniciados"

# Esperar a que los servicios estén listos
echo ""
if ! wait_for_service "db" 40; then
    print_error "La base de datos no inició correctamente"
    docker compose logs db 2>/dev/null || true
    exit 1
fi

if ! wait_for_service "backend" 60; then
    print_error "El backend no inició correctamente"
    docker compose logs backend 2>/dev/null || true
    exit 1
fi

print_success "Todos los servicios están listos"

# 10. Crear Usuario Administrador
echo ""
print_info "Creando usuario administrador..."

# Esperar un poco más para que el backend esté completamente listo
sleep 5

# Obtener credenciales de la DB desde .env
source .env 2>/dev/null || true
DB_USER="${DB_USERNAME:-postgres}"
DB_PASSWORD="${DB_PASSWORD:-password}"
DB_NAME="${DB_NAME:-colleges}"

print_info "Usando DB: ${DB_NAME}, Usuario: ${DB_USER}"

# Obtener school_id
SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM schools WHERE subdomain = 'ccb' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

if [ -z "$SCHOOL_ID" ]; then
    print_warning "Colegio 'ccb' no encontrado, usando primer colegio disponible"
    SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
        "SELECT id FROM schools LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)
fi

# Obtener role_id para admin
ROLE_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM roles WHERE name = 'admin' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

if [ -z "$ROLE_ID" ]; then
    ROLE_ID=1
fi

print_info "School ID: ${SCHOOL_ID}, Role ID: ${ROLE_ID}"

if [ -n "$SCHOOL_ID" ] && [ -n "$ROLE_ID" ]; then
    # Eliminar usuario admin por defecto si existe
    print_info "Limpiando usuarios por defecto..."
    docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
        "DELETE FROM users WHERE email = 'admin@ccb.edu.co';" 2>/dev/null || true

    # Eliminar usuario con el email ingresado si ya existe
    docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
        "DELETE FROM users WHERE email = '${ADMIN_EMAIL}';" 2>/dev/null || true

    # Generar hash de contraseña con Argon2id
    print_info "Generando hash de contraseña con Argon2id..."
    PASSWORD_HASH=$(python3 -c "
import argon2
ph = argon2.PasswordHasher(time_cost=3, memory_cost=4096, parallelism=1)
print(ph.hash('${ADMIN_PASSWORD}'))
" 2>/dev/null)

    if [ -z "$PASSWORD_HASH" ]; then
        print_warning "No se pudo generar hash Argon2, usando alternativa"
        PASSWORD_HASH=$(python3 -c "
import bcrypt
print(bcrypt.hashpw('${ADMIN_PASSWORD}'.encode(), bcrypt.gensalt()).decode())
" 2>/dev/null)
    fi

    if [ -z "$PASSWORD_HASH" ]; then
        print_error "No se pudo generar hash de contraseña"
        print_info "Deberás cambiar la contraseña después del primer login"
        exit 1
    fi

    # Escapar caracteres especiales para SQL
    PASSWORD_HASH_ESCAPED=$(echo "$PASSWORD_HASH" | sed "s/'/''/g")

    # Crear nuevo usuario
    print_info "Creando usuario: ${ADMIN_EMAIL}..."
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
        print_success "Usuario administrador creado exitosamente"
        # Verificar que se creó correctamente
        docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c \
            "SELECT email, name FROM users WHERE email = '${ADMIN_EMAIL}';" 2>/dev/null
    else
        print_error "No se pudo crear el usuario"
        print_info "Resultado: $RESULT"
        exit 1
    fi
else
    print_error "No se pudo obtener información de la base de datos"
    print_info "Asegúrate de que las migraciones se ejecutaron correctamente"
    exit 1
fi

# 11. SSL (solo producción)
if [ "$DEPLOY_MODE" == "production" ]; then
    echo ""
    print_info "Configurando SSL con Certbot..."
    if [ -f "./init-ssl.sh" ]; then
        ./init-ssl.sh
        print_success "SSL configurado"
    else
        print_warning "init-ssl.sh no encontrado"
        print_info "Para configurar SSL manualmente:"
        echo "  docker compose run --rm certbot certonly \\"
        echo "    --webroot \\"
        echo "    --webroot-path=/var/www/certbot \\"
        echo "    --email tu-email@tudominio.com \\"
        echo "    -d tudominio.com"
    fi
fi

# 12. Resumen Final
echo ""
print_header
echo -e "${GREEN}   ✅ ¡Instalación completada!${NC}"
print_header

echo ""
echo "📊 Resumen:"
echo "   🏢 Distribuidor: ${DISTRIBUTOR_NAME}"
echo "   🌐 Modo: ${DEPLOY_MODE}"
if [ -n "$REDIS_PROFILE" ]; then
    echo "   📧 Redis: Habilitado"
fi
if [ -n "$OLLAMA_URL" ]; then
    echo "   🤖 IA/ML: Habilitada (8 endpoints)"
fi

echo ""
echo "👑 Usuario Root (Dueño de la Plataforma):"
echo "   📧 Email: ${ADMIN_EMAIL}"
echo "   🔑 Contraseña: ******** (oculta por seguridad)"

echo ""
echo "🌐 Accesos:"
if [ "$DEPLOY_MODE" == "production" ]; then
    echo "   Frontend: https://tudominio.com"
    echo "   Backend:  https://tudominio.com/api"
else
    echo "   Frontend: http://localhost"
    echo "   Backend:  http://localhost:8080"
    echo "   Health:   http://localhost:8080/health"
fi

echo ""
echo "📋 Comandos útiles:"
echo "   docker compose ps          # Ver estado de contenedores"
echo "   docker compose logs -f     # Ver logs"
echo "   docker compose down -v     # Detener y limpiar todo"
if [ -n "$REDIS_PROFILE" ]; then
    echo "   docker compose --profile with-redis up -d  # Con Redis"
fi

echo ""
echo "🔐 Próximos pasos:"
echo "   1. Inicia sesión en http://localhost con tus credenciales"
echo "   2. Accede a la Consola Root para gestionar colegios"
echo "   3. Crea licencias para cada colegio cliente"
echo "   4. Configura IA/ML en .env si no lo hiciste"
echo "   5. Revisa la documentación en docs/README.md"

echo ""
echo "📚 Documentación:"
echo "   - API Endpoints: API_ENDPOINTS.md (89+ endpoints)"
echo "   - IA/ML: IA_ML_STATUS.md (8 endpoints)"
echo "   - KPIs: KPI_STATUS.md"
echo "   - Seguridad: SEGURIDAD.md"

echo ""
echo "==============================================="
echo "   🎉 SchoolCCB SaaS v1.0.0 - 100% Completado"
echo "==============================================="
echo ""
