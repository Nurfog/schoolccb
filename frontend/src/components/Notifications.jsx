// ============================================
// Componentes de Notificaciones - Fase 6
// ============================================

import React, { useState, useEffect, useCallback } from 'react';
import api from '../api';

// ============================================
// Notification Bell Component
// ============================================

const NotificationBell = ({ onNotificationClick }) => {
  const [unreadCount, setUnreadCount] = useState(0);
  const [loading, setLoading] = useState(false);

  const fetchUnreadCount = useCallback(async () => {
    try {
      const data = await api.get('/api/notifications/unread-count');
      setUnreadCount(data.unread_count);
    } catch (error) {
      console.error('Error fetching unread count:', error);
    }
  }, []);

  useEffect(() => {
    fetchUnreadCount();
    
    // Polling cada 30 segundos para actualizar contador
    const interval = setInterval(fetchUnreadCount, 30000);
    return () => clearInterval(interval);
  }, [fetchUnreadCount]);

  const handleBellClick = async () => {
    setLoading(true);
    try {
      await api.put('/api/notifications/read-all');
      setUnreadCount(0);
      if (onNotificationClick) {
        onNotificationClick();
      }
    } catch (error) {
      console.error('Error marking notifications as read:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleBellClick}
      disabled={loading}
      className="relative p-2 text-white/70 hover:text-white transition-colors"
      title="Notificaciones"
    >
      {/* Icono de campana */}
      <svg
        className="w-6 h-6"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={2}
          d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
        />
      </svg>

      {/* Badge de no leídas */}
      {unreadCount > 0 && (
        <span className="absolute -top-1 -right-1 bg-red-500 text-white text-xs font-bold rounded-full w-5 h-5 flex items-center justify-center animate-pulse">
          {unreadCount > 99 ? '99+' : unreadCount}
        </span>
      )}
    </button>
  );
};

// ============================================
// Notifications List Component
// ============================================

const NotificationsList = ({ isOpen, onClose }) => {
  const [notifications, setNotifications] = useState([]);
  const [loading, setLoading] = useState(false);
  const [page, setPage] = useState(0);
  const limit = 20;

  const fetchNotifications = useCallback(async () => {
    setLoading(true);
    try {
      const data = await api.get(`/api/notifications?limit=${limit}&offset=${page * limit}`);
      setNotifications(prev => page === 0 ? data : [...prev, ...data]);
    } catch (error) {
      console.error('Error fetching notifications:', error);
    } finally {
      setLoading(false);
    }
  }, [page]);

  useEffect(() => {
    if (isOpen) {
      setPage(0);
      setNotifications([]);
      fetchNotifications();
    }
  }, [isOpen]);

  const markAsRead = async (notificationId) => {
    try {
      await api.put(`/api/notifications/${notificationId}/read`);
      setNotifications(prev =>
        prev.map(n =>
          n.id === notificationId ? { ...n, is_read: true, read_at: new Date().toISOString() } : n
        )
      );
    } catch (error) {
      console.error('Error marking notification as read:', error);
    }
  };

  const deleteNotification = async (notificationId) => {
    try {
      await api.delete(`/api/notifications/${notificationId}`);
      setNotifications(prev => prev.filter(n => n.id !== notificationId));
    } catch (error) {
      console.error('Error deleting notification:', error);
    }
  };

  const getNotificationIcon = (type) => {
    const icons = {
      info: 'ℹ️',
      warning: '⚠️',
      error: '❌',
      success: '✅',
      academic: '📚',
      financial: '💰',
    };
    return icons[type] || '📬';
  };

  const getNotificationColor = (type) => {
    const colors = {
      info: 'border-blue-500/30 bg-blue-500/10',
      warning: 'border-yellow-500/30 bg-yellow-500/10',
      error: 'border-red-500/30 bg-red-500/10',
      success: 'border-green-500/30 bg-green-500/10',
      academic: 'border-purple-500/30 bg-purple-500/10',
      financial: 'border-cyan-500/30 bg-cyan-500/10',
    };
    return colors[type] || colors.info;
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex justify-end" onClick={onClose}>
      {/* Overlay */}
      <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" />

      {/* Panel lateral */}
      <div
        className="relative w-full max-w-md bg-[#0f172a] border-l border-white/10 h-full overflow-hidden flex flex-col animate-in slide-in-from-right duration-300"
        onClick={e => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b border-white/10">
          <h2 className="text-xl font-bold text-white">Notificaciones</h2>
          <button
            onClick={onClose}
            className="p-2 text-white/50 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>

        {/* Lista de notificaciones */}
        <div className="flex-1 overflow-y-auto p-4 space-y-3">
          {notifications.length === 0 && !loading ? (
            <div className="text-center py-10 text-white/50">
              <span className="text-4xl mb-4 block">🔔</span>
              <p>No tienes notificaciones</p>
            </div>
          ) : (
            <>
              {notifications.map(notification => (
                <div
                  key={notification.id}
                  className={`p-4 rounded-xl border transition-all ${
                    notification.is_read
                      ? 'border-white/5 bg-white/5'
                      : `${getNotificationColor(notification.notification_type)} border-l-4`
                  }`}
                >
                  <div className="flex items-start justify-between mb-2">
                    <div className="flex items-center space-x-2">
                      <span className="text-xl">{getNotificationIcon(notification.notification_type)}</span>
                      <span className="text-xs font-bold uppercase tracking-wider text-white/60">
                        {notification.notification_type}
                      </span>
                    </div>
                    <span className="text-xs text-white/40">
                      {new Date(notification.created_at).toLocaleDateString('es-ES', {
                        day: 'numeric',
                        month: 'short',
                        hour: '2-digit',
                        minute: '2-digit'
                      })}
                    </span>
                  </div>

                  <h3 className="text-sm font-bold text-white mb-1">
                    {notification.title}
                  </h3>
                  <p className="text-sm text-white/70 mb-3">
                    {notification.message}
                  </p>

                  {/* Actions */}
                  <div className="flex items-center justify-between">
                    {!notification.is_read && (
                      <button
                        onClick={() => markAsRead(notification.id)}
                        className="text-xs text-brand-primary hover:text-brand-primary/80 transition-colors font-medium"
                      >
                        Marcar como leída
                      </button>
                    )}
                    <button
                      onClick={() => deleteNotification(notification.id)}
                      className="text-xs text-red-400 hover:text-red-300 transition-colors ml-auto"
                    >
                      Eliminar
                    </button>
                  </div>
                </div>
              ))}

              {loading && (
                <div className="text-center py-4">
                  <div className="inline-block w-6 h-6 border-2 border-brand-primary border-t-transparent rounded-full animate-spin" />
                </div>
              )}

              {!loading && notifications.length > 0 && (
                <button
                  onClick={() => setPage(prev => prev + 1)}
                  className="w-full py-3 text-sm font-medium text-brand-primary hover:text-brand-primary/80 transition-colors border border-brand-primary/30 rounded-xl"
                >
                  Cargar más
                </button>
              )}
            </>
          )}
        </div>
      </div>
    </div>
  );
};

// ============================================
// Notification Toast Component
// ============================================

const NotificationToast = ({ notification, onClose }) => {
  useEffect(() => {
    const timer = setTimeout(onClose, 5000);
    return () => clearTimeout(timer);
  }, [onClose]);

  const getIcon = (type) => {
    const icons = {
      info: 'ℹ️',
      warning: '⚠️',
      error: '❌',
      success: '✅',
      academic: '📚',
      financial: '💰',
    };
    return icons[type] || '📬';
  };

  return (
    <div className="fixed bottom-4 right-4 z-[100] max-w-sm animate-in slide-in-from-bottom-5 duration-300">
      <div className="bg-[#0f172a] border border-white/10 rounded-2xl p-4 shadow-2xl">
        <div className="flex items-start space-x-3">
          <span className="text-2xl">{getIcon(notification.type)}</span>
          <div className="flex-1">
            <h4 className="text-sm font-bold text-white">{notification.title}</h4>
            <p className="text-sm text-white/70 mt-1">{notification.message}</p>
          </div>
          <button
            onClick={onClose}
            className="text-white/30 hover:text-white transition-colors"
          >
            ✕
          </button>
        </div>
      </div>
    </div>
  );
};

// ============================================
// Export Components
// ============================================

export { NotificationBell, NotificationsList, NotificationToast };
