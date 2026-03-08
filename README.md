# Sistema de Administración de Colegios (SaaS)

Plataforma integral para la gestión escolar diseñada bajo un modelo **SaaS (Software as a Service)**. Este sistema permite a múltiples instituciones gestionar sus procesos académicos, administrativos y financieros de manera eficiente y segura.

## 🚀 Stack Tecnológico

El proyecto utiliza una arquitectura moderna y de alto rendimiento:

*   **Backend Principal:** Rust (Alto rendimiento, lógica de negocio core).
*   **Frontend:** React con Vite (SPA rápida y reactiva).
*   **Base de Datos:** PostgreSQL 16 (Persistencia robusta).
*   **Infraestructura:** Docker & Docker Compose (Orquestación de contenedores).

## 🏗 Arquitectura

El sistema está contenerizado y dividido en servicios modulares:

| Servicio | Contenedor | Puerto Interno | Descripción |
|----------|------------|----------------|-------------|
| **Base de Datos** | `colleges_db` | 5432 | PostgreSQL con volúmenes persistentes. |
| **Backend** | `colleges_backend` | 8080 | API RESTful escrita en Rust. |
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
    docker-compose up --build
    ```

4.  **Acceder a la aplicación:**
    *   Frontend: `http://localhost:80`
    *   Backend API (Rust): `http://localhost:8080`

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