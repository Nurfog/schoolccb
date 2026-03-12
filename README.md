# Sistema de Administración de Colegios (SaaS)

Plataforma integral para la gestión escolar diseñada bajo un modelo **SaaS (Software as a Service)**. Este sistema permite a múltiples instituciones gestionar sus procesos académicos, administrativos y financieros de manera eficiente y segura.

> **📊 Optimizado (Marzo 2026):** Imágenes Docker 60% más pequeñas, connection pool mejorado (5→20), 25+ índices DB, tracing estructurado, CI/CD con auditoría de seguridad. Ver [`OPTIMIZACIONES.md`](OPTIMIZACIONES.md).

## 🚀 Stack Tecnológico

El proyecto utiliza una arquitectura moderna y de alto rendimiento:

*   **Backend Principal:** Rust 1.77 con Actix-web & SQLx (Patrón Repository, RBAC).
*   **Frontend:** React 18 con Vite & Tailwind CSS (UI moderna, Dashboard administrativo).
*   **Base de Datos:** PostgreSQL 16 (Esquema académico versionado con migraciones).
*   **Caché:** Redis 7 (opcional, para sesiones y consultas frecuentes).
*   **Seguridad:** Autenticación JWT, Hashing Argon2id y Claims-based RBAC.
*   **Infraestructura:** Docker & Docker Compose (imágenes distroless optimizadas).
*   **Proxy Reverso:** Nginx (Enrutamiento, balanceo, gzip, rate limiting, SSL).

## 🏗 Arquitectura

El sistema está contenerizado y dividido en servicios modulares:

| Servicio | Contenedor | Puerto | Descripción |
|----------|------------|--------|-------------|
| **Base de Datos** | `colleges_db` | 5432 | PostgreSQL 16 con volúmenes persistentes. |
| **Caché** | `colleges_redis` | 6379 | Redis 7 (opcional, perfil `with-redis`). |
| **Backend** | `colleges_backend` | 8080 | API RESTful Rust Actix-web (imagen ~30MB distroless). |
| **Frontend** | `colleges_frontend` | 80 | Servidor web React/Vite. |
| **Proxy Inverso** | `colleges_nginx` | 80, 443 | Nginx con gzip, security headers, rate limiting. |

## 🛠 Instalación y Despliegue

### Prerrequisitos

*   Docker Engine 20+
*   Docker Compose 2.0+

### Pasos para iniciar (Entorno de Desarrollo)

1.  **Clonar el repositorio:**
    ```bash
    git clone <url-del-repo>
    cd schoolccb
    ```

2.  **Configurar variables de entorno:**
    ```bash
    cp .env.example .env
    # Editar .env con tus valores (JWT_SECRET_KEY, DB_PASSWORD, etc.)
    ```

3.  **Levantar los servicios:**
    ```bash
    # Básico
    docker compose up --build

    # Con Redis (caché)
    docker compose --profile with-redis up --build

    # Producción (con SSL)
    docker compose --profile production up -d
    ```

4.  **Acceder a la aplicación:**
    *   Frontend: `http://localhost`
    *   Backend API: `http://localhost:8080/health`
    *   PostgreSQL: `localhost:5432`
    *   Redis: `localhost:6379` (si está habilitado)

## 📁 Estructura del Proyecto

```
schoolccb/
├── docker-compose.yml          # Orquestación optimizada con healthchecks
├── .env                        # Variables de entorno (producción)
├── .env.example                # Configuración de ejemplo actualizada
├── .env.production             # Plantilla para producción
├── README.md                   # Esta documentación
├── SEGURIDAD.md                # 🔒 Guía de seguridad y despliegue
├── ROADMAP.md                  # Plan de desarrollo
├── OPTIMIZACIONES.md           # 📊 Detalle de optimizaciones implementadas
├── frontend/                   # React + Vite + Tailwind
│   ├── Dockerfile.frontend
│   ├── src/
│   ├── package.json
│   ├── vite.config.js
│   ├── eslint.config.js        # ESLint configurado
│   └── vitest.config.js        # Vitest para tests
├── rust/                       # Rust + Actix-web + SQLx
│   ├── src/
│   │   ├── main.rs             # Tracing estructurado JSON
│   │   ├── handlers.rs
│   │   ├── repository.rs
│   │   └── auth.rs
│   ├── migrations/             # Migraciones SQLx
│   ├── Cargo.toml              # Edition 2021, profile.release optimizado
│   ├── Dockerfile              # Multi-stage → distroless
│   └── .dockerignore
├── nginx/                      # Nginx optimizado
│   ├── nginx.conf              # gzip, security headers, rate limit
│   └── Dockerfile
├── postgres/                   # PostgreSQL 16
│   └── Dockerfile.postgres
├── .github/workflows/          # CI/CD mejorado
│   ├── backend.yml             # fmt, lint, audit, test, build
│   └── frontend.yml            # lint, test, build
└── scripts/
    ├── setup.sh                # Setup interactivo
    └── generate-secrets.sh     # 🔒 Generador de secretos seguros
```

## 🔧 Configuración

### Variables de Entorno Principales

```env
# Base de Datos
DB_USERNAME=admin
DB_PASSWORD=<generar_con_script>
DB_NAME=colleges

# Database Pool (optimizado)
DATABASE_MAX_CONNECTIONS=20
DATABASE_MIN_CONNECTIONS=5
DATABASE_ACQUIRE_TIMEOUT=30
DATABASE_IDLE_TIMEOUT=600
DATABASE_MAX_LIFETIME=1800

# Backend
BACKEND_PORT=8080
RUST_LOG=info

# Autenticación
JWT_SECRET_KEY=<generar_con_script>
SESSION_SECRET=<generar_con_script>

# Frontend
NODE_ENV=production

# Redis (opcional)
REDIS_URL=redis://redis:6379
```

> 🔒 **Importante:** Usa el script `./scripts/generate-secrets.sh` para generar secretos seguros automáticamente.

Ver [`.env.example`](.env.example) para todas las opciones o [`.env.production`](.env.production) para la plantilla de producción.

### 🔒 Seguridad

Para una guía completa de seguridad y despliegue, ver [`SEGURIDAD.md`](SEGURIDAD.md).

## 📊 Estado Actual

| Módulo | Estado | Descripción |
|--------|--------|-------------|
| Infraestructura | ✅ Completo | Docker, Nginx, PostgreSQL, Rust, React/Vite, Redis |
| Autenticación y RBAC | ✅ Completo | JWT, Argon2id, permisos por rol |
| Gestión Académica | ✅ Completo | Cursos, matrículas, calificaciones y asistencia |
| SaaS Multi-colegio | ✅ Completo | Multi-tenancy, subdominios, geo-tagging por país |
| Consola Root | ✅ Completo | Dashboard MRR, gestión de licencias, detalle por institución |
| Importación Masiva CSV | ✅ Completo | `POST /admin/bulk-import` |
| Boletín Estudiantil | ✅ Completo | `GET /academic/my-report-card` |
| Personalización | ✅ Completo | Logo, colores por colegio (marca blanca) |
| Internacionalización | ✅ Completo | ES/EN con `react-i18next` |
| CI/CD | ✅ Mejorado | GitHub Actions: fmt, lint, audit, test, build |
| SSL / HTTPS | ✅ Completo | Certbot + Nginx |
| Caché Redis | ✅ Opcional | Perfil `with-redis` |
| Tracing/Logging | ✅ Mejorado | JSON estructurado con `tracing` |
| **Módulos Premium** | ✅ Implementado | 3 planes (Basic, Premium, Enterprise) con feature flags |
| App Móvil (API) | 🚧 En Progreso | Endpoints optimizados para mobile |

## ⚙️ Instalación Rápida

```bash
# 1. Clonar
git clone <url-del-repo> && cd schoolccb

# 2. Configurar
cp .env.example .env
# Editar .env con valores seguros (especialmente JWT_SECRET_KEY)

# 3. Iniciar
chmod +x setup.sh
./setup.sh

# 4. Acceder
# Frontend: http://localhost
# Usuario: el que creaste en el setup
```

## 🚀 Comandos Útiles

```bash
# Desarrollo
docker compose up --build
docker compose logs -f backend
docker compose exec db psql -U postgres -d colleges

# Con Redis
docker compose --profile with-redis up

# Producción
docker compose --profile production up -d
docker compose down -v  # Limpiar todo (cuidado!)

# Backend (local)
cd rust
cargo run
cargo build --release
cargo clippy -- -D warnings
cargo test

# Frontend (local)
cd frontend
npm install
npm run dev
npm run build
npm run lint
npm run test
```

## 📈 Métricas de Rendimiento

| Métrica | Valor |
|---------|-------|
| Tamaño imagen backend | ~30 MB (distroless) |
| Conexiones DB máximas | 20 (configurable) |
| Healthchecks | 5/5 servicios |
| Índices DB | 25+ |
| Response time objetivo | <500ms |
| Uptime objetivo | 99.9% |

## ⚠️ Problemas Conocidos

- **Migraciones:** Usar `sqlx-cli` para cambios en el esquema. Las migraciones se ejecutan automáticamente al iniciar.
- **CORS:** Configurar `CORS_ORIGIN` en el backend si hay problemas de conexión.
- **Redis:** Es opcional. Habilitar con `--profile with-redis`.

## 🤝 Contribución

1.  Fork el proyecto
2.  Crea una rama (`git checkout -b feature/AmazingFeature`)
3.  Commit tus cambios (`git commit -m 'Add AmazingFeature'`)
4.  Push (`git push origin feature/AmazingFeature`)
5.  Abre un Pull Request

### Requisitos de Código

- **Backend:** `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`
- **Frontend:** `npm run lint`, `npm run test`
- **CI/CD:** Todos los jobs deben pasar (fmt, lint, audit, test, build)

## 📝 Licencia

Todos los derechos reservados. Ver [LICENSE](LICENSE) para más detalles.

---

**📚 Documentación adicional:**
- [Optimizaciones](OPTIMIZACIONES.md) - Detalle de mejoras técnicas implementadas
- [Roadmap](ROADMAP.md) - Plan de desarrollo futuro
- [Módulos Premium](MODULOS_PREMIUM.md) - Planes y features disponibles