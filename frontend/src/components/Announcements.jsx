// ============================================
// Componentes de Comunicados Escolares - Fase 6
// ============================================

import React, { useState, useEffect, useCallback } from 'react';
import api from '../api';

// ============================================
// Announcements List Component
// ============================================

const AnnouncementsList = ({ schoolId }) => {
  const [announcements, setAnnouncements] = useState([]);
  const [loading, setLoading] = useState(true);
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [selectedAnnouncement, setSelectedAnnouncement] = useState(null);

  const categories = [
    { value: 'all', label: 'Todos', icon: '📋' },
    { value: 'urgent', label: 'Urgentes', icon: '🚨' },
    { value: 'informative', label: 'Informativos', icon: '📢' },
    { value: 'academic', label: 'Académicos', icon: '📚' },
    { value: 'administrative', label: 'Administrativos', icon: '🏛️' },
  ];

  const fetchAnnouncements = useCallback(async () => {
    setLoading(true);
    try {
      const categoryParam = selectedCategory !== 'all' ? `&category=${selectedCategory}` : '';
      const data = await api.get(`/api/announcements?limit=50&offset=0${categoryParam}`);
      setAnnouncements(data);
    } catch (error) {
      console.error('Error fetching announcements:', error);
    } finally {
      setLoading(false);
    }
  }, [selectedCategory]);

  useEffect(() => {
    fetchAnnouncements();
  }, [fetchAnnouncements]);

  const confirmReading = async (announcementId) => {
    try {
      await api.post(`/api/announcements/${announcementId}/read`);
      setAnnouncements(prev =>
        prev.map(a =>
          a.id === announcementId ? { ...a, is_read: true } : a
        )
      );
    } catch (error) {
      console.error('Error confirming reading:', error);
    }
  };

  const getCategoryColor = (category) => {
    const colors = {
      urgent: 'bg-red-500/20 border-red-500/50 text-red-400',
      informative: 'bg-blue-500/20 border-blue-500/50 text-blue-400',
      academic: 'bg-purple-500/20 border-purple-500/50 text-purple-400',
      administrative: 'bg-cyan-500/20 border-cyan-500/50 text-cyan-400',
    };
    return colors[category] || colors.informative;
  };

  const getCategoryIcon = (category) => {
    const icons = {
      urgent: '🚨',
      informative: '📢',
      academic: '📚',
      administrative: '🏛️',
    };
    return icons[category] || '📋';
  };

  const getPriorityBadge = (priority) => {
    if (priority >= 3) return { label: 'CRÍTICO', color: 'bg-red-500' };
    if (priority >= 2) return { label: 'ALTA', color: 'bg-orange-500' };
    return { label: 'NORMAL', color: 'bg-blue-500' };
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-20">
        <div className="text-center">
          <div className="w-12 h-12 border-4 border-brand-primary border-t-transparent rounded-full animate-spin mx-auto mb-4" />
          <p className="text-white/50">Cargando comunicados...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Filtros de categoría */}
      <div className="flex flex-wrap gap-2">
        {categories.map(cat => (
          <button
            key={cat.value}
            onClick={() => setSelectedCategory(cat.value)}
            className={`px-4 py-2 rounded-xl text-sm font-medium transition-all ${
              selectedCategory === cat.value
                ? 'bg-brand-primary text-indigo-900 font-bold'
                : 'bg-white/5 text-white/70 hover:bg-white/10 hover:text-white'
            }`}
          >
            <span className="mr-2">{cat.icon}</span>
            {cat.label}
          </button>
        ))}
      </div>

      {/* Lista de comunicados */}
      <div className="space-y-4">
        {announcements.length === 0 ? (
          <div className="text-center py-20 text-white/50">
            <span className="text-6xl mb-4 block">📋</span>
            <p className="text-lg font-medium">No hay comunicados</p>
            <p className="text-sm mt-2">Los comunicados publicados aparecerán aquí</p>
          </div>
        ) : (
          announcements.map(announcement => (
            <div
              key={announcement.id}
              onClick={() => {
                setSelectedAnnouncement(announcement);
                confirmReading(announcement.id);
              }}
              className={`p-6 rounded-2xl border transition-all cursor-pointer hover:scale-[1.02] ${
                announcement.is_read
                  ? 'bg-white/5 border-white/10'
                  : 'bg-gradient-to-r from-brand-primary/10 to-transparent border-brand-primary/30'
              }`}
            >
              <div className="flex items-start justify-between mb-4">
                <div className="flex items-center space-x-3">
                  <span className="text-3xl">{getCategoryIcon(announcement.category)}</span>
                  <div>
                    <div className="flex items-center space-x-2 mb-1">
                      <span className={`px-2 py-1 rounded-lg text-xs font-bold uppercase ${getCategoryColor(announcement.category)}`}>
                        {announcement.category}
                      </span>
                      {announcement.requires_confirmation && (
                        <span className="px-2 py-1 rounded-lg text-xs font-bold uppercase bg-yellow-500/20 border border-yellow-500/50 text-yellow-400">
                          Requiere Confirmación
                        </span>
                      )}
                      {!announcement.is_read && (
                        <span className="px-2 py-1 rounded-lg text-xs font-bold uppercase bg-green-500/20 border border-green-500/50 text-green-400">
                          Nuevo
                        </span>
                      )}
                    </div>
                    <h3 className="text-xl font-bold text-white">
                      {announcement.title}
                    </h3>
                  </div>
                </div>

                <div className="flex items-center space-x-2">
                  {getPriorityBadge(announcement.priority) && (
                    <span className={`px-2 py-1 rounded-lg text-xs font-bold text-white ${getPriorityBadge(announcement.priority).color}`}>
                      {getPriorityBadge(announcement.priority).label}
                    </span>
                  )}
                </div>
              </div>

              {/* Resumen */}
              {announcement.summary && (
                <p className="text-white/70 mb-4 line-clamp-2">
                  {announcement.summary}
                </p>
              )}

              {/* Footer */}
              <div className="flex items-center justify-between text-sm text-white/40">
                <div className="flex items-center space-x-4">
                  <span>👤 {announcement.author_name}</span>
                  <span>📅 {new Date(announcement.published_at).toLocaleDateString('es-ES')}</span>
                </div>
                {announcement.expires_at && (
                  <span className="text-orange-400">
                    ⏰ Vence: {new Date(announcement.expires_at).toLocaleDateString('es-ES')}
                  </span>
                )}
              </div>
            </div>
          ))
        )}
      </div>

      {/* Modal de detalle */}
      {selectedAnnouncement && (
        <AnnouncementDetail
          announcement={selectedAnnouncement}
          onClose={() => setSelectedAnnouncement(null)}
        />
      )}
    </div>
  );
};

// ============================================
// Announcement Detail Modal
// ============================================

const AnnouncementDetail = ({ announcement, onClose }) => {
  const [confirming, setConfirming] = useState(false);

  const handleConfirm = async () => {
    setConfirming(true);
    try {
      await api.post(`/api/announcements/${announcement.id}/read`);
      onClose();
    } catch (error) {
      console.error('Error confirming announcement:', error);
    } finally {
      setConfirming(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4" onClick={onClose}>
      {/* Overlay */}
      <div className="absolute inset-0 bg-black/70 backdrop-blur-sm" />

      {/* Modal */}
      <div
        className="relative w-full max-w-3xl bg-[#0f172a] border border-white/10 rounded-3xl overflow-hidden max-h-[90vh] overflow-y-auto animate-in zoom-in-95 duration-300"
        onClick={e => e.stopPropagation()}
      >
        {/* Header */}
        <div className="relative h-48 bg-gradient-to-r from-brand-primary to-cyan-500">
          <button
            onClick={onClose}
            className="absolute top-4 right-4 p-2 bg-black/20 hover:bg-black/40 rounded-full transition-colors text-white"
          >
            ✕
          </button>
          <div className="absolute bottom-4 left-6 right-6">
            <div className="flex items-center space-x-2 mb-2">
              <span className="px-3 py-1 rounded-lg text-xs font-bold uppercase bg-white/20 text-white">
                {announcement.category}
              </span>
              {announcement.requires_confirmation && (
                <span className="px-3 py-1 rounded-lg text-xs font-bold uppercase bg-yellow-500/80 text-white">
                  Requiere Confirmación
                </span>
              )}
            </div>
            <h2 className="text-3xl font-black text-white">{announcement.title}</h2>
          </div>
        </div>

        {/* Content */}
        <div className="p-6">
          {/* Meta information */}
          <div className="flex items-center justify-between mb-6 pb-6 border-b border-white/10">
            <div className="flex items-center space-x-4 text-sm text-white/60">
              <span>👤 {announcement.author_name}</span>
              <span>📅 {new Date(announcement.published_at).toLocaleDateString('es-ES', {
                weekday: 'long',
                year: 'numeric',
                month: 'long',
                day: 'numeric'
              })}</span>
            </div>
            {announcement.expires_at && (
              <span className="text-sm text-orange-400">
                ⏰ Vence: {new Date(announcement.expires_at).toLocaleDateString('es-ES')}
              </span>
            )}
          </div>

          {/* Body */}
          <div className="prose prose-invert max-w-none mb-6">
            <div className="text-white/80 leading-relaxed whitespace-pre-wrap">
              {announcement.content}
            </div>
          </div>

          {/* Attachments */}
          {announcement.attachment_urls && announcement.attachment_urls.length > 0 && (
            <div className="mb-6">
              <h3 className="text-sm font-bold text-white/60 mb-3">Adjuntos</h3>
              <div className="flex flex-wrap gap-3">
                {announcement.attachment_urls.map((url, index) => (
                  <a
                    key={index}
                    href={url}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="px-4 py-2 bg-white/5 hover:bg-white/10 border border-white/10 rounded-xl text-sm text-white transition-colors flex items-center space-x-2"
                  >
                    <span>📎</span>
                    <span>Adjunto {index + 1}</span>
                  </a>
                ))}
              </div>
            </div>
          )}

          {/* Actions */}
          <div className="flex items-center justify-end space-x-4 pt-6 border-t border-white/10">
            <button
              onClick={onClose}
              className="px-6 py-3 text-white/70 hover:text-white transition-colors font-medium"
            >
              Cerrar
            </button>
            {announcement.requires_confirmation && !announcement.is_read && (
              <button
                onClick={handleConfirm}
                disabled={confirming}
                className="px-8 py-3 bg-brand-primary text-indigo-900 rounded-xl font-bold hover:scale-105 active:scale-95 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {confirming ? 'Confirmando...' : '✅ Confirmar Lectura'}
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

// ============================================
// Create Announcement Form (Admin Only)
// ============================================

const CreateAnnouncement = ({ onSuccess, onCancel }) => {
  const [formData, setFormData] = useState({
    title: '',
    content: '',
    summary: '',
    category: 'informative',
    target_audience: { all: true },
    priority: 1,
    scheduled_at: null,
    expires_at: null,
    allow_comments: false,
    requires_confirmation: false,
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      await api.post('/api/announcements', formData);
      onSuccess();
    } catch (err) {
      setError(err.message || 'Error al crear comunicado');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {error && (
        <div className="p-4 bg-red-500/10 border border-red-500/50 rounded-xl text-red-400">
          {error}
        </div>
      )}

      <div>
        <label className="block text-sm font-medium text-white/70 mb-2">
          Título *
        </label>
        <input
          type="text"
          value={formData.title}
          onChange={e => setFormData({ ...formData, title: e.target.value })}
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-white/30 focus:outline-none focus:ring-2 focus:ring-brand-primary"
          placeholder="Título del comunicado"
          required
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-white/70 mb-2">
          Resumen
        </label>
        <input
          type="text"
          value={formData.summary}
          onChange={e => setFormData({ ...formData, summary: e.target.value })}
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-white/30 focus:outline-none focus:ring-2 focus:ring-brand-primary"
          placeholder="Resumen corto para vista previa"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-white/70 mb-2">
          Contenido *
        </label>
        <textarea
          value={formData.content}
          onChange={e => setFormData({ ...formData, content: e.target.value })}
          rows={8}
          className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white placeholder-white/30 focus:outline-none focus:ring-2 focus:ring-brand-primary resize-none"
          placeholder="Contenido completo del comunicado"
          required
        />
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-sm font-medium text-white/70 mb-2">
            Categoría
          </label>
          <select
            value={formData.category}
            onChange={e => setFormData({ ...formData, category: e.target.value })}
            className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-brand-primary"
          >
            <option value="urgent">🚨 Urgente</option>
            <option value="informative">📢 Informativo</option>
            <option value="academic">📚 Académico</option>
            <option value="administrative">🏛️ Administrativo</option>
          </select>
        </div>

        <div>
          <label className="block text-sm font-medium text-white/70 mb-2">
            Prioridad
          </label>
          <select
            value={formData.priority}
            onChange={e => setFormData({ ...formData, priority: parseInt(e.target.value) })}
            className="w-full px-4 py-3 bg-white/5 border border-white/10 rounded-xl text-white focus:outline-none focus:ring-2 focus:ring-brand-primary"
          >
            <option value={1}>Normal</option>
            <option value={2}>Alta</option>
            <option value={3}>Crítica</option>
          </select>
        </div>
      </div>

      <div className="flex items-center space-x-4">
        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={formData.requires_confirmation}
            onChange={e => setFormData({ ...formData, requires_confirmation: e.target.checked })}
            className="w-4 h-4 rounded border-white/20 bg-white/5 text-brand-primary focus:ring-brand-primary"
          />
          <span className="text-sm text-white/70">Requiere confirmación de lectura</span>
        </label>

        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={formData.allow_comments}
            onChange={e => setFormData({ ...formData, allow_comments: e.target.checked })}
            className="w-4 h-4 rounded border-white/20 bg-white/5 text-brand-primary focus:ring-brand-primary"
          />
          <span className="text-sm text-white/70">Permitir comentarios</span>
        </label>
      </div>

      <div className="flex items-center justify-end space-x-4 pt-6 border-t border-white/10">
        <button
          type="button"
          onClick={onCancel}
          className="px-6 py-3 text-white/70 hover:text-white transition-colors font-medium"
        >
          Cancelar
        </button>
        <button
          type="submit"
          disabled={loading}
          className="px-8 py-3 bg-brand-primary text-indigo-900 rounded-xl font-bold hover:scale-105 active:scale-95 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {loading ? 'Creando...' : 'Crear Comunicado'}
        </button>
      </div>
    </form>
  );
};

// ============================================
// Export Components
// ============================================

export { AnnouncementsList, AnnouncementDetail, CreateAnnouncement };
