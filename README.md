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

## 🚀 Comandos Útiles

```bash
# Construir y levantar todos los servicios
docker compose up --build

# Levantar en segundo plano
docker compose up -d --build

# Ver logs de todos los servicios
docker compose logs -f

# Detener servicios
docker compose down

# Reconstruir un servicio específico
docker compose up --build backend

# Acceder a la base de datos
docker compose exec db psql -U postgres -d colleges
```

## 📊 Estado Actual

- ✅ Infraestructura Docker completa
- ✅ Backend Rust básico con endpoints de salud
- ✅ Frontend React básico con interfaz de bienvenida
- ✅ Proxy reverso Nginx configurado
- ✅ Base de datos PostgreSQL lista
- ✅ Docker Compose funcionando correctamente

## 🤝 Contribución

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 📂 Estructura del Proyecto

```
/schoolccb
├── Dockerfile.postgres     # Dockerfile para la base de datos PostgreSQL
├── Dockerfile.frontend     # Dockerfile para el frontend de React
├── rust/                   # Código fuente del Backend en Rust
│   └── Dockerfile.backend  # Dockerfile para el backend de Rust
├── nginx/                  # Configuración del proxy inverso Nginx
│   ├── Dockerfile
│   └── nginx.conf
├── scripts/                # Scripts de inicialización de DB y utilidades
├── docker-compose.yml
```

## 🔐 Modelo de Negocio (SaaS)

El sistema está diseñado para soportar **Multi-tenancy** (múltiples colegios en una sola instancia) con aislamiento lógico de datos.

*   **Planes:** Mensual y Anual.
*   **Módulos:**
    *   *Base:* Gestión académica, matriculación.
    *   *Premium:* Finanzas, Comunicaciones avanzadas (contenedores/servicios adicionales).

## 🤝 Contribución

1.  Haz un Fork del proyecto.
2.  Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`).
3.  Haz Commit de tus cambios (`git commit -m 'Add some AmazingFeature'`).
4.  Haz Push a la rama (`git push origin feature/AmazingFeature`).
5.  Abre un Pull Request.

## 📄 Licencia

Distribuido bajo la licencia [Nombre de tu Licencia]. Ver `LICENSE` para más información.