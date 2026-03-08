# Roadmap del Proyecto

Este documento describe las fases de desarrollo planificadas para el Sistema de Administración de Colegios SaaS.

## ✅ Fase 1: Cimientos e Infraestructura (Actual)
- [x] Definición de arquitectura (Docker, Rust, Postgres).
- [x] Configuración de `docker-compose.yml` y `Dockerfile` multi-stage.
- [x] Configuración inicial de PostgreSQL (Esquemas, Usuarios).
- [x] "Hola Mundo" conectado entre Frontend -> Backend -> DB.
- [ ] Sistema de Logging y Monitoreo básico.

## 🚧 Fase 2: MVP (Producto Mínimo Viable) - Sistema Base
- [ ] **Autenticación y Autorización:**
    - [ ] Login/Registro (JWT).
    - [ ] Gestión de Roles (Admin, Profesor, Alumno, Padre).
- [ ] **Multi-tenancy:**
    - [ ] Lógica para separar datos por colegio (Schema-based o Row-based).
- [ ] **Módulo Académico Básico:**
    - [ ] Gestión de Alumnos y Profesores.
    - [ ] Cursos y Materias.
    - [ ] Calificaciones simples.

## 📅 Fase 3: Módulos Premium y Expansión
- [ ] **Módulo de Finanzas (Premium):**
    - [ ] Gestión de pensiones y matrículas.
    - [ ] Facturación electrónica.
    - [ ] Reportes de morosidad.
- [ ] **Módulo de Comunicaciones (Premium):**
    - [ ] Chat interno / Notificaciones.
    - [ ] Envío de correos masivos.
- [ ] **Módulo de Asistencia:**
    - [ ] Registro diario y reportes.

## 💰 Fase 4: Funcionalidades Comerciales SaaS
- [ ] **Gestión de Suscripciones:**
    - [ ] Integración con pasarela de pagos (Stripe/MercadoPago).
    - [ ] Automatización de cobros (Mensual/Anual).
    - [ ] Bloqueo automático por falta de pago.
- [ ] **Panel de Super-Admin:**
    - [ ] Métricas globales del SaaS (MRR, Churn).
    - [ ] Alta/Baja de colegios clientes.

## 🚀 Fase 5: Optimización y Escala
- [ ] **Performance:**
    - [ ] Implementación de Caching (Redis).
    - [ ] Optimización de queries en Rust/SQLx.
- [ ] **DevOps:**
    - [ ] CI/CD Pipelines.
    - [ ] Tests automatizados (Unitarios y de Integración).
- [ ] **Mobile:**
    - [ ] API preparada para futura App Móvil.

---

### Leyenda
- [x] Completado
- [ ] Pendiente
- [~] En progreso

### Notas
*   La prioridad actual es estabilizar el contenedor de Rust y la conexión con la base de datos.
*   Los módulos premium se desarrollarán como servicios o librerías modulares para facilitar su activación/desactivación según el plan contratado.