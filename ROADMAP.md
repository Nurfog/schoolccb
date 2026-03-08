# Roadmap del Proyecto

Este documento describe las fases de desarrollo planificadas para el Sistema de Administración de Colegios SaaS.

## ✅ Fase 1: Cimientos e Infraestructura (Completada - Marzo 2026)
- [x] Definición de arquitectura (Docker, Rust, Postgres).
- [x] Configuración de `docker-compose.yml` y `Dockerfile` multi-stage.
- [x] Configuración inicial de PostgreSQL (Esquemas, Usuarios).
- [x] Backend Rust básico con Actix-web (endpoints de salud).
- [x] Frontend React básico con Create React App.
- [x] Proxy reverso Nginx configurado y funcional.
- [x] "Hola Mundo" conectado entre Frontend -> Backend -> DB.
- [x] Sistema de Logging y Monitoreo básico.
- [x] Docker Compose funcionando correctamente.
- [x] Estructura de directorios organizada y limpia.

## 🚧 Fase 2: MVP (Producto Mínimo Viable) - Sistema Base (En Desarrollo)
- [ ] **Autenticación y Autorización:**
    - [ ] Login/Registro (JWT con Rust).
    - [ ] Gestión de Roles (Admin, Profesor, Alumno, Padre).
    - [ ] Middleware de autenticación en Actix-web.
- [ ] **Multi-tenancy:**
    - [ ] Lógica para separar datos por colegio (Schema-based).
    - [ ] Middleware de tenant en el backend.
- [ ] **Módulo Académico Básico:**
    - [ ] Gestión de Alumnos y Profesores (CRUD completo).
    - [ ] Cursos y Materias.
    - [ ] Calificaciones simples.
    - [ ] API RESTful completa con SQLx.
- [ ] **Interfaz de Usuario:**
    - [ ] Dashboard básico para cada rol.
    - [ ] Formularios CRUD con React.
    - [ ] Navegación y routing.

## 📅 Fase 3: Módulos Premium y Expansión (Q2 2026)
- [ ] **Módulo de Finanzas (Premium):**
    - [ ] Gestión de pensiones y matrículas.
    - [ ] Facturación electrónica.
    - [ ] Reportes de morosidad.
    - [ ] Integración con APIs de pago.
- [ ] **Módulo de Comunicaciones (Premium):**
    - [ ] Chat interno / Notificaciones.
    - [ ] Envío de correos masivos.
    - [ ] Sistema de anuncios.
- [ ] **Módulo de Asistencia:**
    - [ ] Registro diario y reportes.
    - [ ] Integración con dispositivos biométricos.
- [ ] **Reportes y Analytics:**
    - [ ] Dashboards con métricas.
    - [ ] Exportación a PDF/Excel.

## 💰 Fase 4: Funcionalidades Comerciales SaaS (Q3 2026)
- [ ] **Gestión de Suscripciones:**
    - [ ] Integración con pasarela de pagos (Stripe/MercadoPago).
    - [ ] Automatización de cobros (Mensual/Anual).
    - [ ] Bloqueo automático por falta de pago.
- [ ] **Panel de Super-Admin:**
    - [ ] Métricas globales del SaaS (MRR, Churn).
    - [ ] Alta/Baja de colegios clientes.
    - [ ] Gestión de planes de suscripción.
- [ ] **Multi-idioma y Localización:**
    - [ ] Soporte para español e inglés.
    - [ ] Configuración regional.

## 🚀 Fase 5: Optimización y Escala (Q4 2026)
- [ ] **Performance:**
    - [ ] Implementación de Caching (Redis).
    - [ ] Optimización de queries en Rust/SQLx.
    - [ ] CDN para assets estáticos.
- [ ] **DevOps:**
    - [ ] CI/CD Pipelines (GitHub Actions).
    - [ ] Tests automatizados (Unitarios y de Integración).
    - [ ] Monitoreo avanzado (Prometheus/Grafana).
- [ ] **Mobile:**
    - [ ] Aplicación React Native.
    - [ ] API optimizada para mobile.
- [ ] **Seguridad:**
    - [ ] Auditorías de seguridad.
    - [ ] Cumplimiento GDPR.
    - [ ] Encriptación de datos sensibles.

## 🎯 Metas por Milestone

### Milestone 1 (Marzo 2026) ✅
- Infraestructura completa funcionando
- "Hola Mundo" end-to-end
- Documentación inicial

### Milestone 2 (Junio 2026)
- MVP funcional con autenticación
- Gestión básica de usuarios
- Primer colegio piloto

### Milestone 3 (Septiembre 2026)
- Módulos premium implementados
- Sistema de pagos integrado
- 5+ colegios en producción

### Milestone 4 (Diciembre 2026)
- Optimización completa
- Aplicación móvil
- 20+ colegios clientes

## 📊 KPIs de Éxito

- **Técnicos:** 99.9% uptime, <500ms response time
- **Negocio:** 50 colegios clientes, MRR objetivo alcanzado
- **Usuario:** Satisfacción >4.5/5, retención >90%

## 🔄 Proceso de Desarrollo

- **Metodología:** Scrum con sprints de 2 semanas
- **Herramientas:** GitHub Projects, Discord para comunicación
- **Testing:** TDD con Rust, Testing Library con React
- **Code Quality:** Clippy, ESLint, Prettier
    - [ ] API preparada para futura App Móvil.

---

### Leyenda
- [x] Completado
- [ ] Pendiente
- [~] En progreso

### Notas
*   La prioridad actual es estabilizar el contenedor de Rust y la conexión con la base de datos.
*   Los módulos premium se desarrollarán como servicios o librerías modulares para facilitar su activación/desactivación según el plan contratado.