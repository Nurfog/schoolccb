# SchoolCCB - Sistema de Administración de Colegios SaaS

## 🎯 Proyecto
Plataforma SaaS para gestión escolar con arquitectura multi-tenant. Backend en Rust (Actix-web), frontend React, PostgreSQL, Nginx proxy.

## 🏗 Arquitectura

### Stack Tecnológico
- **Backend:** Rust + Actix-web 4.x (API RESTful async)
- **Frontend:** React 18.2 + Create React App (SPA)
- **Database:** PostgreSQL 18.3 con SQLx
- **Proxy:** Nginx Alpine (enrutamiento `/api/*` → backend:8080, `/*` → frontend:80)
- **Orquestación:** Docker Compose (4 servicios en red `colleges_network`)

### Servicios
| Servicio | Puerto | Descripción |
|----------|--------|-------------|
| colleges_db | 5432 | PostgreSQL con volúmenes persistentes |
| colleges_backend | 8080 | API Rust con Actix-web |
| colleges_frontend | 80 | React compilado con Nginx |
| colleges_nginx | 80/443 | Proxy reverso y balanceo |

## 🚀 Comandos Esenciales

### Desarrollo
```bash
# Levantar todo
docker compose up --build

# Solo backend
docker compose up backend --build

# Logs en tiempo real
docker compose logs -f

# Acceder a servicios
docker compose exec backend bash
docker compose exec db psql -U postgres -d colleges
```

### Build/Test
```bash
# Backend Rust
cd rust && cargo build --release
cd rust && cargo test

# Frontend React
cd frontend && npm install
cd frontend && npm run build
cd frontend && npm test
```

## 📁 Estructura de Directorios

```
schoolccb/
├── .github/copilot-instructions.md    # ← Este archivo
├── docker-compose.yml                 # Orquestación
├── .env                              # Variables entorno
├── init.sql                          # Schema inicial BD
├── frontend/                         # React SPA
│   ├── Dockerfile.frontend
│   ├── package.json
│   └── src/
├── rust/                            # Backend Rust
│   ├── Dockerfile
│   ├── Cargo.toml
│   └── src/main.rs
├── postgres/                        # Config BD
│   └── Dockerfile.postgres
├── nginx/                           # Proxy reverso
│   ├── Dockerfile
│   └── nginx.conf
└── scripts/init-db/                 # Scripts SQL auto-ejecutados
```

## 💻 Convenciones de Código

### Rust Backend
```rust
// Handlers con macros de Actix
#[actix_web::get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        message: "Server running".to_string(),
    })
}

// Main con configuración
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = App::new()
        .service(health)
        .service(index);

    HttpServer::new(move || app)
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
```

### React Frontend
```jsx
// Componentes funcionales
import './App.css';

function App() {
  return (
    <div className="App">
      <header>
        <h1>Sistema de Colegios</h1>
      </header>
    </div>
  );
}

export default App;
```

### Nombres
- **Contenedores:** `colleges_<servicio>` (colleges_backend, colleges_db)
- **Paquetes:** kebab-case (colegio-backend, colegio-frontend)
- **Rutas API:** `/api/v1/<recurso>/<acción>`

## ⚙️ Variables de Entorno

```env
# Base de Datos
DB_USERNAME=postgres
DB_PASSWORD=changeme123!
DB_NAME=colleges

# Backend
BACKEND_PORT=8080
RUST_LOG=info

# Frontend
NODE_ENV=development
```

## 🔧 Decisiones Arquitectónicas

### Multi-stage Docker
- **Rust:** Builder stage → Debian slim runtime (reduce ~500MB)
- **React:** Node build → Nginx alpine runtime
- **PostgreSQL:** Custom Dockerfile para configuración inicial

### Multi-tenancy Planificado
- **Schema-based:** Un schema por colegio en PostgreSQL
- **Middleware:** Tenant ID en headers/context
- **Fase 2:** Implementar separación de datos

### API Design
- **RESTful:** Recursos con CRUD estándar
- **JSON:** Request/response bodies
- **CORS:** Configurado en Nginx proxy
- **Health checks:** `/health` endpoint básico

## 🚨 Pitfalls Comunes

### ⚠️ Edition 2024 Rust
```toml
# Cambiar si hay errores de compilación
edition = "2021"  # en lugar de "2024"
```

### ⚠️ init.sql Vacío
- Sin schema inicial → BD vacía
- **Solución:** Implementar migraciones SQLx en Fase 2

### ⚠️ CORS No Configurado
```rust
// Agregar en main.rs si hay errores de CORS
.use(actix_cors::Cors::default())
```

### ⚠️ Password Default
- `changeme123!` visible en código
- **Solución:** Usar `.env` obligatorio en producción

### ⚠️ Frontend Sin API
- React no conecta al backend
- **Solución:** Implementar fetch/axios en Fase 2

## 📋 Checklist Desarrollo

### Antes de Commit
- [ ] `docker compose build` funciona
- [ ] `docker compose up` inicia sin errores
- [ ] Logs limpios (sin errores obvios)
- [ ] Health check `/health` responde OK

### Cambios en Backend
- [ ] Nuevo endpoint documentado
- [ ] Tests unitarios si aplica
- [ ] CORS configurado si nuevo origen

### Cambios en Frontend
- [ ] `npm run build` funciona
- [ ] Componentes exportados correctamente
- [ ] CSS modular (no global)

### Cambios en BD
- [ ] Scripts en `scripts/init-db/`
- [ ] Schema versionado
- [ ] Rollback plan si destructivo

## 🎯 Fases de Desarrollo

### ✅ Fase 1 (Completada)
Infraestructura "Hola Mundo" end-to-end funcionando

### 🚧 Fase 2 (Activa)
MVP con autenticación JWT y multi-tenancy básico

### 📅 Fase 3-5
Módulos premium, SaaS billing, mobile, escalabilidad

## 🤖 Instrucciones para Agentes IA

### Código Nuevo
- **Rust:** `rust/src/main.rs` (handlers) o crear módulos
- **React:** `frontend/src/` (mantener estructura flat)
- **DB:** `scripts/init-db/` (auto-ejecutados en startup)

### Patrones
- **API:** JSON responses con `{status, message, data?}`
- **Error handling:** Result<T, Error> en Rust
- **Logging:** `tracing` crate en backend

### Testing
- **Rust:** `cargo test` (unitarios)
- **React:** `npm test` (Jest)
- **Integration:** `docker compose up` + manual testing

### Deployment
- **Local:** `docker compose up --build`
- **Prod:** Configurar secrets, SSL, monitoring

---

*Última actualización: Marzo 2026 - Infraestructura completa, Fase 2 en desarrollo*