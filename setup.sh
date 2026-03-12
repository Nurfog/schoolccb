#!/bin/bash

# ============================================
# Colegio CCB - Interactive Setup Script v3.0
# ============================================
# Setup automatizado para producción y desarrollo
# ============================================

set -e

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ============================================
# Funciones Auxiliares
# ============================================

print_header() {
    echo -e "${CYAN}"
    echo "==============================================="
    echo "   🛡️  Colegio CCB - Setup v3.0"
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

wait_for_service() {
    local service=$1
    local max_attempts=30
    local attempt=1
    
    print_info "Esperando a que $service esté listo..."
    
    while [ $attempt -le $max_attempts ]; do
        if docker compose ps | grep -q "$service.*healthy"; then
            print_success "$service está listo"
            return 0
        fi
        sleep 2
        attempt=$((attempt + 1))
    done
    
    print_warning "Timeout esperando $service, continuando..."
    return 1
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
echo "  1) Desarrollo (HTTP)"
echo "  2) Producción (HTTPS con SSL)"
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

# 5. Configuración de .env
echo ""
print_info "Configurando archivo .env..."

if [ ! -f .env ]; then
    cp .env.example .env
    print_success "Archivo .env creado desde .env.example"
fi

# Generar JWT_SECRET_KEY si no existe
if ! grep -q "^JWT_SECRET_KEY=" .env || grep -q "^JWT_SECRET_KEY=$" .env; then
    JWT_SECRET=$(openssl rand -base64 64 2>/dev/null || python3 -c "import secrets; print(secrets.token_urlsafe(64))" 2>/dev/null || echo "default_secret_key_change_in_production")
    sed -i "s|^JWT_SECRET_KEY=.*|JWT_SECRET_KEY=${JWT_SECRET}|" .env 2>/dev/null || \
    echo "JWT_SECRET_KEY=${JWT_SECRET}" >> .env
    print_success "JWT_SECRET_KEY generada"
fi

# Actualizar variables de entorno
sed -i "s|^RUST_ENVIRONMENT=.*|RUST_ENVIRONMENT=${DEPLOY_MODE}|" .env 2>/dev/null || true
sed -i "s|^NODE_ENV=.*|NODE_ENV=${DEPLOY_MODE}|" .env 2>/dev/null || true

# Agregar DISTRIBUTOR_NAME
if grep -q "^DISTRIBUTOR_NAME=" .env; then
    sed -i "s|^DISTRIBUTOR_NAME=.*|DISTRIBUTOR_NAME=\"${DISTRIBUTOR_NAME}\"|" .env 2>/dev/null || true
else
    echo "DISTRIBUTOR_NAME=\"${DISTRIBUTOR_NAME}\"" >> .env
fi

print_success "Archivo .env configurado"

# 6. Credenciales del Dueño de la Plataforma (Root)
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
read -s -p "🔑 Contraseña: " ADMIN_PASSWORD
echo ""
read -s -p "🔑 Confirmar Contraseña: " ADMIN_PASSWORD_CONFIRM
echo ""

if [ "$ADMIN_PASSWORD" != "$ADMIN_PASSWORD_CONFIRM" ]; then
    print_error "Las contraseñas no coinciden"
    exit 1
fi

if [ ${#ADMIN_PASSWORD} -lt 8 ]; then
    print_error "La contraseña debe tener al menos 8 caracteres"
    exit 1
fi

# 7. Iniciar Servicios
echo ""
print_info "Iniciando servicios Docker..."
docker compose up -d --build

# Esperar a que los servicios estén listos
echo ""
wait_for_service "db"
wait_for_service "backend"

# 8. Crear Usuario Administrador
echo ""
print_info "Creando usuario administrador..."

# Esperar un poco más para que el backend esté completamente listo
sleep 5

# Obtener credenciales de la DB desde .env
source .env 2>/dev/null || true
DB_USER="${DB_USERNAME:-admin}"
DB_PASSWORD="${DB_PASSWORD:-password}"
DB_NAME="${DB_NAME:-colegios_main}"

print_info "Usando DB: ${DB_NAME}, Usuario: ${DB_USER}"

# Obtener school_id (usamos el colegio ccb por defecto o el primero disponible)
SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM schools WHERE subdomain = 'ccb' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

if [ -z "$SCHOOL_ID" ]; then
    print_warning "Colegio 'ccb' no encontrado, usando primer colegio disponible"
    SCHOOL_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
        "SELECT id FROM schools LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)
fi

# Obtener role_id para admin (role_id=1 es admin por defecto)
ROLE_ID=$(docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -t -c \
    "SELECT id FROM roles WHERE name = 'admin' LIMIT 1;" 2>/dev/null | tr -d ' ' | head -1)

# Si no encuentra admin, usar role_id=1 por defecto
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
    
    # Generar hash de contraseña
    # Intentamos usar Python con argon2-cffi si está disponible
    PASSWORD_HASH=""
    
    if command -v python3 &> /dev/null; then
        # Verificar si argon2 está instalado, si no, intentar instalarlo
        if ! python3 -c "import argon2" 2>/dev/null; then
            print_info "Instalando argon2-cffi para generar hash seguro..."
            pip3 install argon2-cffi -q 2>/dev/null || \
            pip install argon2-cffi -q 2>/dev/null || \
            print_warning "No se pudo instalar argon2-cffi"
        fi
        
        # Intentar generar hash
        if python3 -c "import argon2" 2>/dev/null; then
            PASSWORD_HASH=$(python3 -c "
import argon2
ph = argon2.PasswordHasher(time_cost=3, memory_cost=4096, parallelism=1)
print(ph.hash('${ADMIN_PASSWORD}'))
" 2>/dev/null || echo "")
        fi
    fi
    
    # Si no se pudo generar hash, usar hash por defecto
    if [ -z "$PASSWORD_HASH" ]; then
        print_warning "No se pudo generar hash Argon2, usando contraseña por defecto"
        print_info "Deberás cambiar la contraseña después del primer login"
        PASSWORD_HASH='$argon2id$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$i769B7jI77yXqV6N7z6w7w'
        ADMIN_PASSWORD="admin123"
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
    fi
else
    print_error "No se pudo obtener información de la base de datos"
    print_info "Asegúrate de que las migraciones se ejecutaron correctamente"
fi

# 10. SSL (solo producción)
if [ "$DEPLOY_MODE" == "production" ]; then
    echo ""
    print_info "Configurando SSL con Certbot..."
    if [ -f "./init-ssl.sh" ]; then
        ./init-ssl.sh
        print_success "SSL configurado"
    else
        print_warning "init-ssl.sh no encontrado"
    fi
fi

# 11. Resumen Final
echo ""
print_header
echo -e "${GREEN}   ✅ ¡Instalación completada!${NC}"
print_header

echo ""
echo "📊 Resumen:"
echo "   🏢 Distribuidor: ${DISTRIBUTOR_NAME}"
echo "   🌐 Modo: ${DEPLOY_MODE}"

echo ""
echo "👑 Usuario Root (Dueño de la Plataforma):"
echo "   📧 Email: ${ADMIN_EMAIL}"
echo "   🔑 Contraseña: ${ADMIN_PASSWORD}"
if [ "$ADMIN_PASSWORD" = "admin123" ]; then
    echo "   ⚠️  IMPORTANTE: Cambia esta contraseña después del primer login"
fi

echo ""
echo "🌐 Accesos:"
echo "   Frontend: http://localhost"
echo "   Backend:  http://localhost:8080"
echo "   Health:   http://localhost:8080/health"

echo ""
echo "📋 Comandos útiles:"
echo "   docker compose ps          # Ver estado de contenedores"
echo "   docker compose logs -f     # Ver logs"
echo "   docker compose down -v     # Detener y limpiar todo"

echo ""
echo "🔐 Próximos pasos:"
echo "   1. Inicia sesión en http://localhost con tus credenciales"
echo "   2. Accede a la Consola Root para gestionar colegios"
echo "   3. Crea licencias para cada colegio cliente"

echo ""
echo "==============================================="

# ============================================
# Función para crear rol Root y permisos SaaS
# ============================================
setup_root_role() {
    print_info "Configurando rol Root y permisos SaaS..."

    # Obtener credenciales desde .env
    source .env 2>/dev/null || true
    DB_USER="${DB_USERNAME:-admin}"
    DB_PASSWORD="${DB_PASSWORD:-password}"
    DB_NAME="${DB_NAME:-colegios_main}"

    docker exec -e PGPASSWORD="${DB_PASSWORD}" -i colleges_db psql -U "${DB_USER}" -d "${DB_NAME}" -c "
    -- Crear rol root si no existe
    INSERT INTO roles (id, name, description)
    VALUES (100, 'root', 'Dueño de la plataforma - Acceso total a todas las funciones SaaS')
    ON CONFLICT (name) DO NOTHING;
    
    -- Insertar permisos SaaS si no existen
    INSERT INTO permissions (name, description)
    VALUES 
        ('saas:manage_platform', 'Gestionar toda la plataforma SaaS'),
        ('saas:view_all_schools', 'Ver todos los colegios'),
        ('saas:view_all_licenses', 'Ver todas las licencias'),
        ('saas:manage_billing', 'Gestionar facturación'),
        ('saas:view_analytics', 'Ver analítica avanzada'),
        ('admin:system_config', 'Configuración del sistema')
    ON CONFLICT (name) DO NOTHING;
    
    -- Asignar permisos al rol root
    INSERT INTO role_permissions (role_id, permission_id)
    SELECT 
        (SELECT id FROM roles WHERE name = 'root'),
        p.id
    FROM permissions p
    ON CONFLICT DO NOTHING;
    
    -- Actualizar colegio como sistema
    UPDATE schools SET is_system_admin = true WHERE subdomain = 'ccb';
    "
    
    print_success "Rol Root configurado correctamente"
}
