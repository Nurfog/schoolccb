# 🔒 Guía de Seguridad y Despliegue

## 📋 Cambios Realizados (Marzo 2026)

### 1. **Secretos Seguros Generados**
- ✅ `JWT_SECRET_KEY`: 64 caracteres aleatorios
- ✅ `SESSION_SECRET`: 64 caracteres aleatorios  
- ✅ `DB_PASSWORD`: 32 caracteres aleatorios

### 2. **Mejoras en `.env`**
- ✅ Contraseñas seguras generadas
- ✅ CORS restringido a dominio específico
- ✅ Variables de pool de conexiones añadidas
- ✅ Rate limiting configurado
- ✅ SMTP configurado (deshabilitado por defecto)
- ✅ SSL habilitado con rutas de certificados

### 3. **Nginx - Security Headers**
- ✅ X-Frame-Options (previene clickjacking)
- ✅ X-Content-Type-Options (previene MIME sniffing)
- ✅ X-XSS-Protection (filtro XSS)
- ✅ Referrer-Policy (controla información de referer)
- ✅ Content-Security-Policy (previene XSS e inyección)
- ✅ Strict-Transport-Security (forza HTTPS)
- ✅ Rate limiting (10 req/s para API, 30 req/s general)
- ✅ Gzip compression habilitado
- ✅ Keepalive connections para mejor rendimiento

### 4. **Docker Compose - Aislamiento de Red**
- ✅ Red interna `db_internal` para db/redis/backend
- ✅ Base de datos sin acceso directo desde exterior
- ✅ Mejor segmentación de servicios

## 🚀 Pasos para Despliegue en Producción

### 1. Generar Secretos Únicos

```bash
# JWT Secret (64 chars)
openssl rand -base64 64 | tr -d '\n' && echo ""

# Session Secret (64 chars)
openssl rand -base64 64 | tr -d '\n' && echo ""

# Database Password (32 chars)
openssl rand -base64 48 | tr -d '\n' && echo ""
```

### 2. Configurar Dominio

Editar `.env`:
```env
CORS_ORIGIN=https://school-ccb.xyz,https://www.school-ccb.xyz
DOMAIN_NAME=school-ccb.xyz
CERTBOT_EMAIL=tu-email@school-ccb.xyz
```

### 3. Configurar SMTP (Opcional pero recomendado)

```env
SMTP_ENABLED=true
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USER=tu-email@gmail.com
SMTP_PASSWORD=tu-app-password
SMTP_FROM_NAME=SchoolCCB
SMTP_FROM_EMAIL=noreply@school-ccb.xyz
```

> **Nota para Gmail:** Usar "App Password" desde https://myaccount.google.com/apppasswords

### 4. Iniciar en Producción

```bash
# Con SSL y Redis
docker compose --profile production --profile with-redis up -d --build

# Ver logs
docker compose logs -f

# Verificar health
curl http://localhost/health
```

### 5. Obtener Certificado SSL

```bash
# Ejecutar Certbot manualmente la primera vez
docker compose run --rm certbot certonly --webroot \
  --webroot-path=/var/www/certbot \
  --email admin@school-ccb.xyz \
  --agree-tos \
  --no-eff-email \
  -d school-ccb.xyz \
  -d www.school-ccb.xyz
```

## 📊 Checklist de Seguridad

| Item | Estado | Descripción |
|------|--------|-------------|
| 🔒 JWT Secret Seguro | ✅ | 64 caracteres aleatorios |
| 🔒 DB Password Seguro | ✅ | 32 caracteres aleatorios |
| 🔒 Session Secret Seguro | ✅ | 64 caracteres aleatorios |
| 🔒 CORS Restringido | ✅ | Solo dominio específico |
| 🔒 Rate Limiting | ✅ | 10 req/s API, 30 req/s general |
| 🔒 Security Headers | ✅ | 6 headers implementados |
| 🔒 Red Interna DB | ✅ | Aislamiento de red |
| 🔒 SSL/TLS | ⚠️ | Configurar Certbot |
| 🔒 SMTP | ⚠️ | Configurar si se requiere |
| 🔒 Backup | ⚠️ | Implementar script |

## 🔍 Comandos de Verificación

```bash
# Verificar configuración de Nginx
docker compose exec nginx nginx -t

# Ver conexiones activas a DB
docker compose exec db psql -U admin -d colegios_main -c "SELECT count(*) FROM pg_stat_activity;"

# Ver logs de seguridad
docker compose logs nginx | grep -i "error\|denied"

# Verificar rate limiting
for i in {1..15}; do curl -s -o /dev/null -w "%{http_code}\n" http://localhost/api/health; done

# Ver uso de recursos
docker compose stats
```

## 🛡 Mejores Prácticas

### 1. **Actualizaciones**
```bash
# Actualizar dependencias Rust (mensual)
cd rust && cargo update

# Actualizar dependencias Frontend (mensual)
cd frontend && npm update

# Actualizar imágenes Docker (trimestral)
docker compose pull
```

### 2. **Backups**
```bash
# Backup manual de PostgreSQL
docker compose exec db pg_dump -U admin colegios_main > backup_$(date +%Y%m%d).sql

# Restaurar backup
docker compose exec -T db psql -U admin colegios_main < backup_20260311.sql
```

### 3. **Monitoreo**
- Revisar logs diariamente: `docker compose logs --tail=100`
- Verificar health checks: `docker compose ps`
- Monitorear recursos: `docker stats`

## ⚠️ Problemas Comunes

### CORS Errors
```env
# Asegurar que el dominio en CORS_ORIGIN coincida exactamente
CORS_ORIGIN=https://school-ccb.xyz
```

### Rate Limiting Too Strict
```nginx
# Ajustar en nginx.conf si es necesario
limit_req_zone $binary_remote_addr zone=api_limit:10m rate=20r/s;
```

### SSL Certificate Issues
```bash
# Forzar renovación
docker compose run --rm certbot renew --force-renewal
```

## 📞 Soporte

Para issues de seguridad, contactar: admin@school-ccb.xyz

---

**Última actualización:** Marzo 2026  
**Versión:** 1.0.0
