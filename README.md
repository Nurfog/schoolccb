# Sistema de Administración de Colegios (SaaS)

Plataforma integral para la gestión escolar diseñada bajo un modelo **SaaS (Software as a Service)**. Este sistema permite a múltiples instituciones gestionar sus procesos académicos, administrativos y financieros de manera eficiente y segura.

## 🚀 Stack Tecnológico

El proyecto utiliza una arquitectura moderna y de alto rendimiento:

*   **Backend Principal:** Rust con Actix-web (Alto rendimiento, lógica de negocio core).
*   **Frontend:** React con Create React App (SPA rápida y reactiva).
*   **Base de Datos:** PostgreSQL 16 (Persistencia robusta).
*   **Infraestructura:** Docker & Docker Compose (Orquestación de contenedores).
*   **Proxy Reverso:** Nginx (Enrutamiento y balanceo de carga).

## 🏗 Arquitectura

El sistema está contenerizado y dividido en servicios modulares:

| Servicio | Contenedor | Puerto Interno | Descripción |
|----------|------------|----------------|-------------|
| **Base de Datos** | `colleges_db` | 5432 | PostgreSQL con volúmenes persistentes. |
| **Backend** | `colleges_backend` | 8080 | API RESTful escrita en Rust con Actix-web. |
| **Frontend** | `colleges_frontend` | 80 | Servidor web que entrega la aplicación React. |
| **Proxy Inverso** | `colleges_nginx` | 80, 443 | Nginx para enrutamiento de tráfico y balanceo de carga. |

## 🛠 Instalación y Despliegue

### Prerrequisitos

*   Docker Engine
*   Docker Compose

### Pasos para iniciar (Entorno de Desarrollo)

1.  **Clonar el repositorio:**
    ```bash
    git clone <url-del-repo>
    cd schoolccb
    ```

2.  **Configurar variables de entorno:**
    Crea un archivo `.env` basado en el ejemplo (si existe) o utiliza los valores por defecto del `docker-compose.yml`.

3.  **Levantar los servicios:**
    ```bash
    docker compose up --build
    ```

4.  **Acceder a la aplicación:**
    *   Frontend: `http://localhost:80`
    *   Backend API (Rust): `http://localhost:8080`
    *   Base de Datos: `localhost:5432`

## 📁 Estructura del Proyecto

```
schoolccb/
├── docker-compose.yml          # Orquestación de servicios
├── init.sql                    # Script de inicialización BD
├── .env                        # Variables de entorno
├── .env.example               # Ejemplo de configuración
├── README.md                   # Documentación del proyecto
├── ROADMAP.md                  # Plan de desarrollo
├── frontend/                   # Código fuente React
│   ├── Dockerfile.frontend     # Build del frontend
│   ├── public/
│   ├── src/
│   ├── package.json
│   └── package-lock.json
├── rust/                       # Código fuente Rust
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile              # Build del backend
├── postgres/                   # Configuración PostgreSQL
│   └── Dockerfile.postgres     # Configuración BD
├── nginx/                      # Configuración proxy reverso
│   ├── nginx.conf
│   └── Dockerfile
├── scripts/                    # Scripts auxiliares
│   └── init-db/               # Scripts de inicialización BD
└── src/                        # Código fuente adicional (legacy)
```

## 🔧 Configuración

### Variables de Entorno

Crea un archivo `.env` en la raíz del proyecto:

```env
# Base de Datos
DB_USERNAME=postgres
DB_PASSWORD=changeme123!
DB_NAME=colleges

# Backend
BACKEND_PORT=8080
RUST_LOG=info

# Frontend
NODE_ENV=production
```

## � Estado Actual

- ✅ **Infraestructura completa:** Docker Compose funcionando con 4 servicios
- ✅ **Backend Rust:** API básica con Actix-web y endpoints `/health`, `/`
- ✅ **Frontend React:** Aplicación básica compilada con Nginx
- ✅ **Base de datos:** PostgreSQL configurado y accesible
- ✅ **Proxy reverso:** Nginx enrutando correctamente
- ⚠️ **Pendiente:** Schema inicial de BD (init.sql vacío)
- ⚠️ **Atención:** Edition 2024 de Rust puede causar incompatibilidades

## 🚀 Comandos Útiles

```bash
# Desarrollo completo
docker compose up --build

# Solo un servicio
docker compose up backend --build

# Ver logs en tiempo real
docker compose logs -f backend
docker compose logs -f frontend

# Acceder a servicios
docker compose exec backend bash
docker compose exec db psql -U postgres -d colleges

# Detener todo
docker compose down
```

## 🔧 Configuración

### Variables de Entorno

Crea un archivo `.env` en la raíz del proyecto:

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

### Problemas Conocidos

- **Edition 2024 Rust:** Si hay errores de compilación, cambiar a `edition = "2021"` en `rust/Cargo.toml`
- **Base de datos vacía:** Implementar schema inicial en `init.sql` para Fase 2
- **CORS:** Configurar en backend si hay problemas de conexión desde frontend

## 🤝 Contribución

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Todos los derechos reservados. Este proyecto es propiedad exclusiva y no puede ser copiado, distribuido o utilizado sin autorización expresa por escrito.