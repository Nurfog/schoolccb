# 🚀 Guía de Producción - SchoolCCB SaaS

## 📁 Archivos de Configuración

### 1. `.env` (Producción) ⚠️ CONFIDENCIAL
**NO subir a Git** - Ya está en `.gitignore`

**Variables CRÍTICAS que debes configurar:**

```bash
# Base de Datos - CAMBIAR
DB_PASSWORD=tu_password_seguro_aqui

# JWT - Generar clave segura
JWT_SECRET_KEY=$(openssl rand -base64 64)

# Dominio - CAMBIAR
CORS_ORIGINS=https://tudominio.com

# Stripe (opcional para pagos)
STRIPE_SECRET_KEY=sk_live_...
STRIPE_PUBLISHABLE_KEY=pk_live_...

# Email (opcional para notificaciones)
SMTP_USER=tu_email@tudominio.com
SMTP_PASSWORD=tu_password_de_aplicacion

# Root Console - TUS datos
PLATFORM_OWNER_EMAIL=tu_email@tudominio.com
ROOT_NOTIFICATION_EMAIL=tu_email@tudominio.com
```

---

## 🔐 Generar JWT_SECRET_KEY

```bash
# Opción 1: OpenSSL
openssl rand -base64 64

# Opción 2: Python
python3 -c "import secrets; print(secrets.token_urlsafe(64))"

# Opción 3: Node.js
node -e "console.log(require('crypto').randomBytes(64).toString('base64'))"
```

---

## 🏫 Diferencia: Billing por Colegio vs Root Console

### 📊 Perspectiva por Colegio

Cada colegio tiene su propia instancia de billing:

| Endpoint | Propósito | Acceso |
|----------|-----------|--------|
| `GET /api/billing/plans` | Ver planes disponibles | Admin de colegio |
| `GET /api/billing/my-plan` | Ver SU plan actual | Admin de colegio |
| `POST /api/billing/checkout` | Suscribirse/upgrade | Admin de colegio |

**Flujo:**
1. Admin del colegio inicia sesión
2. Va a "Planes" en el sidebar
3. Ve los 3 planes (Basic, Premium, Enterprise)
4. Selecciona y paga con Stripe
5. Su colegio queda actualizado

---

### 👑 Root Console (TÚ como dueño)

Tú ves TODOS los colegios y licencias:

| Endpoint | Propósito | Acceso |
|----------|-----------|--------|
| `GET /api/saas/dashboard` | Métricas globales (MRR, colegios) | SuperAdmin |
| `GET /api/saas/licenses` | Todas las licencias | SuperAdmin |
| `GET /api/saas/schools/stats` | Colegios con estadísticas | SuperAdmin |
| `POST /api/saas/licenses` | Crear/actualizar licencia | SuperAdmin |
| `PUT /api/saas/schools/{id}` | Gestionar colegio | SuperAdmin |

**Flujo:**
1. Tú inicias sesión como SuperAdmin
2. Ves el dashboard con MRR total
3. Gestionas TODOS los colegios
4. Creas licencias manualmente si es necesario
5. Ves qué licencias están por vencer

---

## 📊 Endpoints Root (Dueño de la Plataforma)

### Dashboard Global
```bash
curl -H "Authorization: Bearer TU_TOKEN_ROOT" \
  https://api.tudominio.com/api/saas/dashboard
```

**Respuesta:**
```json
{
  "total_schools": 15,
  "mrr": 1487.00,
  "annual_forecast": 17844.00,
  "active_licenses": 12,
  "expiring_licenses": 3
}
```

### Ver Todas las Licencias
```bash
curl -H "Authorization: Bearer TU_TOKEN_ROOT" \
  https://api.tudominio.com/api/saas/licenses
```

### Crear Licencia Manualmente
```bash
curl -X POST \
  -H "Authorization: Bearer TU_TOKEN_ROOT" \
  -H "Content-Type: application/json" \
  -d '{
    "school_id": "uuid-del-colegio",
    "plan_type": "premium",
    "status": "active",
    "expiry_date": "2026-12-31T23:59:59Z",
    "auto_renew": true
  }' \
  https://api.tudominio.com/api/saas/licenses
```

---

## 🎯 Configuración de Stripe

### 1. Crear Productos en Stripe Dashboard

Ve a https://dashboard.stripe.com/products y crea:

| Nombre | Precio | Recurrencia |
|--------|--------|-------------|
| Basic Monthly | $49 | Mensual |
| Basic Yearly | $490 | Anual |
| Premium Monthly | $99 | Mensual |
| Premium Yearly | $990 | Anual |
| Enterprise Monthly | $249 | Mensual |
| Enterprise Yearly | $2,490 | Anual |

### 2. Obtener Price IDs

Cada producto tendrá un ID como `price_xxxxx`. Copia estos IDs.

### 3. Configurar Webhooks

En Stripe Dashboard → Developers → Webhooks:

1. Add endpoint: `https://api.tudominio.com/api/billing/stripe-webhook`
2. Seleccionar eventos:
   - `checkout.session.completed`
   - `customer.subscription.updated`
   - `customer.subscription.deleted`
   - `invoice.payment_failed`
3. Copiar el `Signing Secret` (whsec_xxxxx)

### 4. Actualizar .env

```bash
STRIPE_SECRET_KEY=sk_live_xxxxx
STRIPE_PUBLISHABLE_KEY=pk_live_xxxxx
STRIPE_WEBHOOK_SECRET=whsec_xxxxx
```

---

## 📧 Configuración de Email (SMTP)

### Gmail (Recomendado para empezar)

1. Ve a https://myaccount.google.com/apppasswords
2. Genera una "App Password"
3. Configura en `.env`:

```bash
SMTP_ENABLED=true
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tu_email@gmail.com
SMTP_PASSWORD=app_password_generada
SMTP_FROM_NAME=Colegio CCB
SMTP_FROM_EMAIL=noreply@tudominio.com
```

### SendGrid (Producción)

```bash
SMTP_ENABLED=true
SMTP_HOST=smtp.sendgrid.net
SMTP_PORT=587
SMTP_USER=apikey
SMTP_PASSWORD=tu_sendgrid_api_key
```

---

## 🗄️ Base de Datos

### Ejecutar Migraciones

```bash
# Conectar al contenedor
docker compose exec backend sqlx migrate run

# Verificar migraciones
docker compose exec backend sqlx migrate info
```

### Crear Usuario Root (SuperAdmin)

```bash
# 1. Generar hash de contraseña
docker compose exec backend ./colegio-backend --hash-password "tu_password_seguro"

# 2. Insertar en DB
docker compose exec db psql -U postgres -d colleges -c "
INSERT INTO users (school_id, role_id, name, email, password_hash)
VALUES (
  (SELECT id FROM schools WHERE subdomain = 'ccb'),
  (SELECT id FROM roles WHERE name = 'admin'),
  'Tu Nombre',
  'tu_email@tudominio.com',
  '\$argon2id\$v=19\$m=4096,t=3,p=1\$...'
)
ON CONFLICT (email) DO NOTHING;
"
```

---

## 🚀 Despliegue

### 1. Construir imágenes

```bash
docker compose build
```

### 2. Iniciar servicios

```bash
# Modo desarrollo
docker compose up -d

# Con Redis (caché)
docker compose --profile with-redis up -d

# Producción (con SSL)
docker compose --profile production up -d
```

### 3. Verificar logs

```bash
# Todos los logs
docker compose logs -f

# Solo backend
docker compose logs -f backend

# Solo nginx
docker compose logs -f nginx
```

### 4. Health checks

```bash
# Backend
curl http://localhost:8080/health

# Frontend
curl http://localhost/

# Nginx
curl http://localhost/nginx-health
```

---

## 🔐 SSL/HTTPS (Producción)

### Certbot Automático

```bash
# Ejecutar script de SSL
chmod +x init-ssl.sh
./init-ssl.sh
```

### Configurar dominio

En tu DNS provider (Cloudflare, GoDaddy, etc.):

```
A record: tudominio.com → IP_del_servidor
A record: www.tudominio.com → IP_del_servidor
```

---

## 📊 Monitoreo

### Ver MRR y métricas

```bash
# Desde la API (como root)
curl -H "Authorization: Bearer TU_TOKEN" \
  https://api.tudominio.com/api/saas/dashboard | jq
```

### Logs de auditoría

```bash
# Ver logs del backend
docker compose logs backend | grep -E "(INFO|WARN|ERROR)"

# Buscar pagos
docker compose logs backend | grep "Payment"
```

---

## 🐛 Troubleshooting

### Error: "Database connection failed"

```bash
# Verificar que DB esté corriendo
docker compose ps db

# Ver logs de DB
docker compose logs db

# Reiniciar DB
docker compose restart db
```

### Error: "JWT_SECRET_KEY not set"

```bash
# Verificar que esté en .env
grep JWT_SECRET_KEY .env

# Reiniciar backend
docker compose restart backend
```

### Error: "Stripe no está configurado"

```bash
# Verificar variables de Stripe
grep STRIPE .env

# Reiniciar backend
docker compose restart backend
```

---

## 📋 Checklist de Producción

- [ ] `.env` configurado con valores seguros
- [ ] `JWT_SECRET_KEY` generada aleatoriamente
- [ ] `DB_PASSWORD` cambiado del valor por defecto
- [ ] `CORS_ORIGINS` configurado con tu dominio
- [ ] SSL configurado con Certbot
- [ ] Stripe configurado (si usas pagos)
- [ ] SMTP configurado (si usas emails)
- [ ] Usuario Root creado
- [ ] Migraciones ejecutadas
- [ ] Backups configurados
- [ ] Monitoreo de logs activo
- [ ] `.env` en `.gitignore` (ya está)

---

## 📞 Soporte

### Documentación Adicional

- [Módulos Premium](MODULOS_PREMIUM.md) - Planes y features
- [Optimizaciones](OPTIMIZACIONES.md) - Mejoras técnicas
- [Roadmap](ROADMAP.md) - Futuro del proyecto

### Contacto

- **Email:** soporte@schoolccb.com
- **Ventas:** ventas@schoolccb.com

---

*Guía actualizada: Marzo 2026*
