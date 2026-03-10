import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import api from './api';
import Login from './Login';
import Modal from './Modal';
import BulkImport from './BulkImport';
import BrandingConfig from './BrandingConfig';
import LanguageSwitcher from './LanguageSwitcher';
import { CourseForm, TeacherForm, StudentForm, EnrollmentForm, GradeForm, AttendanceForm, SchoolForm, SchoolEditForm, LicenseForm } from './Forms';

const SidebarItem = ({ icon, label, active, onClick }) => (
  <button
    onClick={onClick}
    className={`w-full flex items-center space-x-3 px-4 py-3 rounded-xl transition-all duration-300 ${active
      ? 'bg-brand-primary text-indigo-900 font-bold shadow-lg shadow-brand-primary/20'
      : 'text-blue-100/70 hover:bg-white/10 hover:text-white'
      }`}
  >
    <span className="text-xl">{icon}</span>
    <span className="text-sm tracking-wide">{label}</span>
  </button>
);

function App() {
  const { t } = useTranslation();
  const [user, setUser] = useState(api.getUser());
  const [activeTab, setActiveTab] = useState('dashboard');
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [modalType, setModalType] = useState('create');
  const [selectedItem, setSelectedItem] = useState(null);
  const [selectedStudent, setSelectedStudent] = useState(null);
  const [actionLoading, setActionLoading] = useState(false);
  const [notification, setNotification] = useState(null);
  const [allStudents, setAllStudents] = useState([]);
  const [grades, setGrades] = useState([]);
  const [viewingGrades, setViewingGrades] = useState(false);
  const [activePeriod, setActivePeriod] = useState(null);
  const [reportCard, setReportCard] = useState([]);
  const [saasStats, setSaasStats] = useState(null);
  const [rootStats, setRootStats] = useState(null);
  const [licenses, setLicenses] = useState([]);
  const [schoolStats, setSchoolStats] = useState([]);
  const [countries, setCountries] = useState([]);
  const [selectedSchool, setSelectedSchool] = useState(null);
  const [school, setSchool] = useState(api.getSchool());

  useEffect(() => {
    // If we have school data from login or fetch, apply branding
    const targetSchool = school || (user && user.school); // Assuming the login returns user.school
    if (targetSchool) {
      if (targetSchool.primary_color) {
        document.documentElement.style.setProperty('--primary-color', targetSchool.primary_color);
      }
      if (targetSchool.secondary_color) {
        document.documentElement.style.setProperty('--secondary-color', targetSchool.secondary_color);
      }
    }
  }, [school, user]);

  const fetchData = async () => {
    if (activeTab === 'settings') return;

    if ((activeTab === 'dashboard' || activeTab === 'saas_finances') && user.is_system_admin) {
      try {
        const stats = await api.get('/saas/dashboard');
        setRootStats(stats);
      } catch (err) {
        // fallback to old stats endpoint if needed
        try { const s = await api.get('/saas/stats'); setSaasStats(s); } catch { }
      }
      return;
    }

    if (activeTab === 'saas_licenses') {
      setLoading(true);
      try {
        const result = await api.get('/saas/licenses');
        setLicenses(result);
      } catch (err) { console.error(err); }
      finally { setLoading(false); }
      return;
    }

    if (activeTab === 'schools_health') {
      setLoading(true);
      try {
        const result = await api.get('/saas/schools/stats');
        setSchoolStats(result);
      } catch (err) { console.error(err); }
      finally { setLoading(false); }
      return;
    }

    setLoading(true);
    try {
      if (activeTab === 'report_card') {
        const result = await api.get('/academic/my-report-card');
        setReportCard(result);
      } else if (activeTab === 'managed_schools' || activeTab === 'school_detail') {
        if (countries.length === 0) fetchCountries();
        const result = await api.get('/saas/schools');
        setData(result);
      } else if (activeTab === 'school_detail' && selectedSchool) {
        const result = await api.get(`/saas/schools/${selectedSchool.id}`);
        setSelectedSchool(result);
      } else if (activeTab !== 'dashboard') {
        const endpoint = activeTab === 'course_members'
          ? `/academic/courses/${selectedItem.id}/students`
          : `/academic/${activeTab}`;
        const result = await api.get(endpoint);
        setData(result);
      }
    } catch (err) {
      console.error(`Error fetching ${activeTab}:`, err);
      setData([]);
    } finally {
      setLoading(false);
    }
  };

  const fetchActivePeriod = async () => {
    if (user.is_system_admin) return;
    try {
      const period = await api.get('/academic/active-period');
      setActivePeriod(period);
    } catch (err) {
      console.error('Error fetching active period:', err);
    }
  };

  useEffect(() => {
    if (user) {
      fetchActivePeriod();
      fetchData();
      if (activeTab === 'course_members' && selectedItem) {
        fetchCourseGrades(selectedItem.id);
      }
    }
  }, [user, activeTab, selectedItem]);

  const fetchStudentsForEnrollment = async () => {
    try {
      const students = await api.get('/academic/students');
      setAllStudents(students);
    } catch (err) {
      console.error('Error fetching students for enrollment:', err);
    }
  };

  const fetchCourseGrades = async (courseId) => {
    try {
      const result = await api.get(`/academic/courses/${courseId}/grades`);
      setGrades(result);
    } catch (err) {
      console.error('Error fetching grades:', err);
    }
  };

  useEffect(() => {
    if (notification) {
      const timer = setTimeout(() => setNotification(null), 5000);
      return () => clearTimeout(timer);
    }
  }, [notification]);

  const handleLogout = () => {
    api.logout();
    setUser(null);
  };

  const handleAction = async (method, endpoint, formData, successMessage) => {
    setActionLoading(true);
    try {
      await api[method](endpoint, formData);
      setNotification({ type: 'success', message: successMessage });
      setIsModalOpen(false);
      fetchData();
      if (activeTab === 'course_members' && selectedItem) fetchCourseGrades(selectedItem.id);
    } catch (err) {
      setNotification({ type: 'error', message: err.message });
    } finally {
      setActionLoading(false);
    }
  };

  if (!user) {
    return <Login onLogin={setUser} />;
  }

  const fetchCountries = async () => {
    try {
      const result = await api.get('/saas/countries');
      setCountries(result);
    } catch (err) {
      console.error('Error fetching countries:', err);
    }
  };

  const getModalTitle = () => {
    if (modalType === 'enroll') return `Matricular en ${selectedItem?.name}`;
    if (modalType === 'grade') return 'Registrar Calificación';
    if (modalType === 'attendance') return 'Pasar Lista';
    if (activeTab === 'courses') return 'Crear Nuevo Curso';
    if (activeTab === 'teachers') return 'Registrar Nuevo Profesor';
    if (activeTab === 'students') return 'Inscribir Nuevo Estudiante';
    if (activeTab === 'managed_schools') return 'Registrar Nuevo Colegio';
    return 'Nuevo Recurso';
  };

  return (
    <div className="flex h-screen bg-[#0a0f1e] text-white font-sans overflow-hidden">
      {/* Notifications */}
      {notification && (
        <div className={`fixed top-8 right-8 z-[100] p-4 rounded-2xl shadow-2xl border animate-in slide-in-from-right-10 duration-500 ${notification.type === 'success'
          ? 'bg-cyan-500/10 border-cyan-500/50 text-cyan-400'
          : 'bg-red-500/10 border-red-500/50 text-red-400'
          }`}>
          <div className="flex items-center space-x-3">
            <span className="text-xl">{notification.type === 'success' ? '✅' : '❌'}</span>
            <span className="font-bold text-sm">{notification.message}</span>
          </div>
        </div>
      )}

      {/* Sidebar */}
      <aside className="w-72 bg-white/5 border-r border-white/10 p-6 flex flex-col space-y-8 backdrop-blur-xl">
        <div className="flex items-center space-x-3 px-2 cursor-pointer group" onClick={() => { setActiveTab('dashboard'); setViewingGrades(false); }}>
          {school?.logo_url ? (
            <img src={school.logo_url} alt="Logo" className="w-10 h-10 object-contain rounded-xl shadow-lg shadow-brand-primary/20 transform group-hover:rotate-12 transition-transform" />
          ) : (
            <div className={`w-10 h-10 ${user?.is_system_admin ? 'bg-gradient-to-br from-violet-500 to-cyan-500' : 'bg-brand-primary'} rounded-xl flex items-center justify-center font-black text-indigo-900 text-xl shadow-lg shadow-brand-primary/20 transform group-hover:rotate-12 transition-transform`}>
              {user?.is_system_admin ? '⚡' : 'C'}
            </div>
          )}
          <span className="text-2xl font-black tracking-tighter">
            {user.is_system_admin ? <><span className="text-violet-400">Root</span><span className="text-cyan-400">Console</span></> : <>Colegio<span className="text-brand-primary">CCB</span></>}
          </span>
        </div>

        <nav className="flex-1 space-y-2">
          <SidebarItem icon="📊" label={t('sidebar.dashboard')} active={activeTab === 'dashboard' && !viewingGrades} onClick={() => { setActiveTab('dashboard'); setViewingGrades(false); }} />

          {user.is_system_admin ? (
            <>
              <SidebarItem icon="🏫" label={t('sidebar.managed_schools')} active={activeTab === 'managed_schools'} onClick={() => setActiveTab('managed_schools')} />
              <SidebarItem icon="💳" label={t('sidebar.licenses')} active={activeTab === 'saas_licenses'} onClick={() => setActiveTab('saas_licenses')} />
              <SidebarItem icon="💰" label={t('sidebar.saas_finances')} active={activeTab === 'saas_finances'} onClick={() => setActiveTab('saas_finances')} />
              <SidebarItem icon="🏥" label="Salud" active={activeTab === 'schools_health'} onClick={() => setActiveTab('schools_health')} />
              <SidebarItem icon="🌎" label="Países" active={activeTab === 'countries'} onClick={() => setActiveTab('countries')} />
            </>
          ) : (
            user.role === 'alumno' ? (
              <SidebarItem icon="📋" label={t('sidebar.report_card')} active={activeTab === 'report_card'} onClick={() => setActiveTab('report_card')} />
            ) : (
              <>
                <SidebarItem icon="📚" label={t('sidebar.courses')} active={activeTab === 'courses' || activeTab === 'course_members'} onClick={() => { setActiveTab('courses'); setViewingGrades(false); }} />
                <SidebarItem icon="👨‍🏫" label={t('sidebar.teachers')} active={activeTab === 'teachers'} onClick={() => { setActiveTab('teachers'); setViewingGrades(false); }} />
                <SidebarItem icon="👥" label={t('sidebar.students')} active={activeTab === 'students'} onClick={() => { setActiveTab('students'); setViewingGrades(false); }} />
                {user.role === 'admin' && (
                  <>
                    <SidebarItem icon="📤" label={t('sidebar.import')} active={activeTab === 'bulk_import'} onClick={() => setActiveTab('bulk_import')} />
                    <SidebarItem icon="🎨" label={t('sidebar.branding')} active={activeTab === 'branding'} onClick={() => setActiveTab('branding')} />
                  </>
                )}

              </>
            )
          )}

          <SidebarItem icon="⚙️" label="Config" active={activeTab === 'settings'} onClick={() => { setActiveTab('settings'); setViewingGrades(false); }} />
        </nav>

        {activePeriod && !user.is_system_admin && (
          <div className="bg-cyan-500/5 border border-cyan-500/20 p-4 rounded-2xl">
            <p className="text-[9px] font-black uppercase tracking-widest text-cyan-400/60 mb-1">Periodo Actual</p>
            <p className="text-xs font-bold truncate">{activePeriod.name}</p>
          </div>
        )}

        <div className="flex flex-col space-y-3 pt-4 border-t border-white/10">
          <LanguageSwitcher />
          <div className="group bg-white/5 p-3 rounded-2xl border border-white/10 flex items-center justify-between transition-all hover:bg-white/10">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-indigo-500 rounded-full flex items-center justify-center font-bold">
                {user.name.substring(0, 2).toUpperCase()}
              </div>
              <div>
                <p className="text-xs font-bold">{user.name}</p>
                <p className="text-[10px] text-cyan-400 font-black uppercase tracking-widest">{user.role}</p>
              </div>
            </div>
            <button
              onClick={handleLogout}
              className="p-2 text-white/30 hover:text-red-400 transition-colors"
              title="Cerrar Sesión"
            >
              🚪
            </button>
          </div>
        </div>
      </aside>

      {/* Main Content */}
      <main className="flex-1 overflow-y-auto bg-gradient-to-br from-transparent via-[#0f172a] to-[#1e293b] p-10">
        <header className="flex justify-between items-center mb-10">
          <div>
            <div className="flex items-center space-x-4 mb-2">
              {activeTab === 'course_members' && (
                <button onClick={() => setActiveTab('courses')} className="text-cyan-400 hover:text-cyan-300 transition-colors text-2xl">
                  ←
                </button>
              )}
              {activeTab === 'school_detail' && (
                <button onClick={() => setActiveTab('managed_schools')} className="text-cyan-400 hover:text-cyan-300 transition-colors text-2xl">
                  ←
                </button>
              )}
              <h2 className="text-4xl font-black tracking-tight capitalize">
                {activeTab === 'dashboard' ? (user.is_system_admin ? 'SaaS Administration' : 'Panel de Control') :
                  activeTab === 'report_card' ? 'Mi Reporte Académico' :
                    activeTab === 'course_members' ? `Aula: ${selectedItem?.name}` :
                      activeTab === 'branding' ? 'Marca e Identidad' :
                        activeTab === 'saas_licenses' ? 'Licencias SaaS' :
                          activeTab === 'managed_schools' ? 'Gestión de Colegios' :
                            activeTab === 'school_detail' ? `Gestionar: ${selectedSchool?.name}` :
                              activeTab}
              </h2>
            </div>
            <p className="text-blue-100/50 text-sm font-medium italic">
              {user.is_system_admin
                ? 'Monitoreo global de licencias, facturación y nuevas entidades escolares.'
                : activeTab === 'course_members' ? 'Gestión de alumnos, asistencia y calificaciones.'
                  : activeTab === 'report_card' ? 'Resumen global de notas y asistencia por curso.'
                    : 'Sección administrativa del colegio central Bogotá.'}
            </p>
          </div>
          <div className="flex space-x-4">
            {activeTab === 'course_members' && (
              <button
                onClick={() => setViewingGrades(!viewingGrades)}
                className={`px-6 py-3 rounded-xl font-bold transition-all border ${viewingGrades
                  ? 'bg-cyan-500/20 border-cyan-500/50 text-cyan-400'
                  : 'bg-white/5 border-white/10 hover:bg-white/10'
                  }`}
              >
                {viewingGrades ? '📋 Ver Alumnos' : '📈 Ver Calificaciones'}
              </button>
            )}

            {(user.is_system_admin && activeTab === 'managed_schools') ||
              (user.role === 'admin' && !user.is_system_admin && activeTab !== 'dashboard' && activeTab !== 'settings' && !viewingGrades) ||
              (user.role === 'profesor' && (activeTab === 'courses' || activeTab === 'course_members') && !viewingGrades) ? (
              <button
                onClick={() => {
                  if (activeTab === 'course_members') {
                    fetchStudentsForEnrollment();
                    setModalType('enroll');
                  } else if (activeTab === 'managed_schools') {
                    fetchCountries();
                    setModalType('create');
                  } else {
                    setModalType('create');
                  }
                  setIsModalOpen(true);
                }}
                className="bg-brand-primary text-indigo-950 px-6 py-2.5 rounded-xl font-black text-sm tracking-widest hover:scale-105 active:scale-95 transition-all shadow-lg shadow-brand-primary/20"
              >
                {activeTab === 'managed_schools' ? '+ Nuevo Colegio' :
                  activeTab === 'course_members' ? '+ Matricular Alumno' :
                    `+ Nuevo ${activeTab === 'courses' ? 'Curso' : activeTab === 'teachers' ? 'Profesor' : 'Estudiante'}`}
              </button>
            ) : null}
          </div>
        </header>

        {activeTab === 'dashboard' && (
          user.is_system_admin ? (
            <div className="space-y-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
              <div className="bg-gradient-to-r from-violet-600/20 via-indigo-600/20 to-cyan-600/20 border border-violet-500/30 rounded-3xl p-6 flex items-center justify-between backdrop-blur-md">
                <div>
                  <p className="text-[10px] font-black uppercase tracking-widest text-violet-400 mb-1">Root Platform Console</p>
                  <h3 className="text-2xl font-black">Bienvenido, {user.name} ⚡</h3>
                  <p className="text-blue-100/50 text-sm mt-1">Vista global del ecosistema. {new Date().toLocaleString('es-CO', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}</p>
                </div>
                <div className="text-6xl opacity-20">🌐</div>
              </div>
              <div className="grid grid-cols-2 lg:grid-cols-3 gap-5">
                {[
                  { icon: '🏫', label: 'Colegios Cliente', value: rootStats?.total_schools ?? '—', color: 'cyan' },
                  { icon: '💰', label: 'MRR (Recurrente Mensual)', value: rootStats?.mrr ? `$${parseFloat(rootStats.mrr).toLocaleString()}` : '—', color: 'indigo' },
                  { icon: '📈', label: 'Forecast Anual', value: rootStats?.annual_forecast ? `$${parseFloat(rootStats.annual_forecast).toLocaleString()}` : '—', color: 'emerald' },
                  { icon: '💳', label: 'Licencias Activas', value: rootStats?.active_licenses ?? '—', color: 'emerald' },
                  { icon: '🧪', label: 'Trials Activos', value: rootStats?.trial_licenses ?? '—', color: 'yellow' },
                  { icon: '⚠️', label: 'Licencias por Vencer', value: rootStats?.expiring_licenses ?? '—', color: 'orange' },
                ].map(({ icon, label, value, color }) => (
                  <div key={label} className="bg-white/5 border border-white/10 p-6 rounded-3xl backdrop-blur-md hover:bg-white/10 transition-all group">
                    <span className="text-3xl block mb-3 group-hover:scale-110 transition-transform">{icon}</span>
                    <p className="text-blue-100/50 text-[10px] font-black uppercase tracking-widest mb-1">{label}</p>
                    <p className="text-4xl font-black">{value}</p>
                  </div>
                ))}
              </div>
              <div className="grid grid-cols-3 gap-4">
                {[
                  { label: 'Ver Licencias', tab: 'saas_licenses', icon: '💳' },
                  { label: 'Ver Finanzas', tab: 'saas_finances', icon: '💰' },
                  { label: 'Gestionar Colegios', tab: 'managed_schools', icon: '🏫' },
                ].map(({ label, tab, icon }) => (
                  <button key={tab} onClick={() => setActiveTab(tab)}
                    className="bg-white/5 hover:bg-white/10 border border-white/10 rounded-2xl p-5 flex items-center space-x-3 transition-all text-left group">
                    <span className="text-2xl">{icon}</span>
                    <span className="text-sm font-bold group-hover:text-cyan-400 transition-colors">{label}</span>
                  </button>
                ))}
              </div>
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
              <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md hover:bg-white/10 transition-all cursor-default group">
                <span className="text-4xl block mb-4 group-hover:scale-110 transition-transform">🎓</span>
                <p className="text-blue-100/60 text-xs font-black uppercase tracking-widest mb-1">Total Estudiantes</p>
                <p className="text-4xl font-black">1,240</p>
              </div>
              <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md hover:bg-white/10 transition-all cursor-default group">
                <span className="text-4xl block mb-4 group-hover:scale-110 transition-transform">🏫</span>
                <p className="text-blue-100/60 text-xs font-black uppercase tracking-widest mb-1">Cursos Activos</p>
                <p className="text-4xl font-black">48</p>
              </div>
              <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md hover:bg-white/10 transition-all cursor-default group">
                <span className="text-4xl block mb-4 group-hover:scale-110 transition-transform">📈</span>
                <p className="text-blue-100/60 text-xs font-black uppercase tracking-widest mb-1">Rendimiento Prom.</p>
                <p className="text-4xl font-black">84%</p>
              </div>
            </div>
          )
        )}

        {/* Managed Schools View (SuperAdmin only) */}
        {activeTab === 'managed_schools' && user.is_system_admin && (
          <div className="animate-in fade-in slide-in-from-bottom-5 duration-500">
            <div className="bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-md">
              <table className="w-full text-left">
                <thead className="bg-white/5 border-b border-white/10">
                  <tr>
                    <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Nombre del Colegio</th>
                    <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Subdominio</th>
                    <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Tipo</th>
                    <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-right">Acciones</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {data.length > 0 ? data.map(school => (
                    <tr key={school.id} className="hover:bg-white/5 transition-colors group">
                      <td className="p-6">
                        <div className="flex items-center space-x-3">
                          <div className="w-8 h-8 bg-cyan-500/20 text-cyan-400 rounded-lg flex items-center justify-center font-black text-xs">
                            {school.name.substring(0, 1)}
                          </div>
                          <span className="font-bold">{school.name}</span>
                        </div>
                      </td>
                      <td className="p-6 text-blue-100/60 font-mono text-sm">{school.subdomain}.ccb.edu.co</td>
                      <td className="p-6 text-center">
                        <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase ${school.is_system_admin ? 'bg-indigo-500/20 text-indigo-400' : 'bg-green-500/20 text-green-400'}`}>
                          {school.is_system_admin ? 'Sistema' : 'Cliente'}
                        </span>
                      </td>
                      <td className="p-6 text-right">
                        <button
                          onClick={() => { setSelectedSchool(school); setActiveTab('school_detail'); }}
                          className="text-cyan-400 hover:text-cyan-300 transition-colors text-xs font-black uppercase tracking-widest"
                        >
                          Gestionar →
                        </button>
                      </td>
                    </tr>
                  )) : (
                    <tr>
                      <td colSpan="4" className="p-20 text-center text-blue-100/20 italic">No hay colegios registrados en el ecosistema.</td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* Licenses View (Root) */}
        {activeTab === 'saas_licenses' && user.is_system_admin && (
          <div className="animate-in fade-in slide-in-from-bottom-5 duration-500">
            <div className="bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-md">
              <table className="w-full text-left">
                <thead className="bg-white/5 border-b border-white/10">
                  <tr>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Colegio</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Plan</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Estado</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Vencimiento</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Auto-Renovar</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Acciones</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {loading ? (
                    <tr><td colSpan="5" className="p-20 text-center text-blue-100/30">Cargando licencias...</td></tr>
                  ) : licenses.length > 0 ? licenses.map(lic => {
                    const statusColors = {
                      active: 'bg-emerald-500/20 text-emerald-400',
                      trial: 'bg-yellow-500/20 text-yellow-400',
                      expired: 'bg-red-500/20 text-red-400',
                      suspended: 'bg-slate-500/20 text-slate-400',
                    };
                    const daysLeft = Math.ceil((new Date(lic.expiry_date) - new Date()) / 86400000);
                    return (
                      <tr key={lic.id} className="hover:bg-white/5 transition-colors">
                        <td className="p-5">
                          <div className="flex items-center space-x-3">
                            <div className="w-8 h-8 bg-violet-500/20 text-violet-400 rounded-lg flex items-center justify-center font-black text-xs">
                              {lic.school_name?.substring(0, 1)}
                            </div>
                            <span className="font-bold text-sm">{lic.school_name}</span>
                          </div>
                        </td>
                        <td className="p-5">
                          <span className="bg-indigo-500/10 text-indigo-300 px-3 py-1 rounded-full text-[10px] font-black uppercase">{lic.plan_type}</span>
                        </td>
                        <td className="p-5 text-center">
                          <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase ${statusColors[lic.status] || 'bg-white/10 text-white/50'}`}>
                            {lic.status}
                          </span>
                        </td>
                        <td className="p-5">
                          <div>
                            <p className="text-sm font-bold">{new Date(lic.expiry_date).toLocaleDateString('es-CO')}</p>
                            <p className={`text-[10px] font-black ${daysLeft < 0 ? 'text-red-400' : daysLeft < 30 ? 'text-yellow-400' : 'text-blue-100/30'}`}>
                              {daysLeft < 0 ? `Vencida hace ${Math.abs(daysLeft)}d` : `${daysLeft}d restantes`}
                            </p>
                          </div>
                        </td>
                        <td className="p-5 text-center">
                          <button
                            onClick={() => { setSelectedItem(lic); setModalType('license'); setIsModalOpen(true); }}
                            className="bg-white/10 hover:bg-white/20 p-2 rounded-xl transition-all"
                          >
                            ⚙️
                          </button>
                        </td>
                      </tr>
                    );
                  }) : (
                    <tr><td colSpan="5" className="p-20 text-center text-blue-100/20 italic">No hay licencias registradas en el sistema.</td></tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}
        {/* Finances View (Root) */}
        {activeTab === 'saas_finances' && user.is_system_admin && (
          <div className="space-y-8 animate-in fade-in slide-in-from-bottom-5 duration-500">
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md">
                <h3 className="text-xl font-bold mb-6 flex items-center space-x-2">
                  <span>💰</span> <span>Ingresos Mensuales por Plan</span>
                </h3>
                <div className="space-y-4">
                  {['enterprise', 'premium', 'basic'].map(plan => {
                    const amount = rootStats?.revenue_by_plan?.[plan] || 0;
                    const price = plan === 'enterprise' ? 249 : plan === 'premium' ? 99 : 49;
                    const count = amount > 0 ? Math.round(parseFloat(amount) / price) : 0;
                    return (
                      <div key={plan} className="flex justify-between items-center p-4 bg-white/5 rounded-2xl border border-white/5 group hover:bg-white/10 transition-all">
                        <div>
                          <p className="text-xs font-black uppercase tracking-widest text-blue-100/40 mb-1">{plan}</p>
                          <p className="text-sm font-bold text-cyan-400">{count} Clientes Activos</p>
                        </div>
                        <div className="text-right">
                          <p className="text-xl font-black">${parseFloat(amount).toLocaleString()}</p>
                          <p className="text-[10px] uppercase font-bold text-blue-100/20">/ mes</p>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>

              <div className="space-y-8">
                <div className="bg-gradient-to-br from-indigo-500/20 to-purple-500/20 border border-indigo-500/20 p-8 rounded-3xl backdrop-blur-md relative overflow-hidden">
                  <div className="absolute top-0 right-0 p-8 opacity-10 text-8xl">📈</div>
                  <h3 className="text-xs font-black uppercase tracking-widest text-indigo-300 mb-2">Forecast Anual Directo</h3>
                  <p className="text-5xl font-black mb-2">${rootStats?.annual_forecast ? parseFloat(rootStats.annual_forecast).toLocaleString() : '—'}</p>
                  <p className="text-sm text-indigo-100/40 font-medium italic">Proyección basada en {rootStats?.active_licenses || 0} suscripciones vigentes.</p>
                </div>

                <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md">
                  <h3 className="text-xl font-bold mb-6 flex items-center space-x-2">
                    <span>💳</span> <span>Métricas de Retención</span>
                  </h3>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="bg-white/5 p-4 rounded-2xl border border-white/5 text-center">
                      <p className="text-[10px] font-black uppercase text-blue-100/30 mb-1">Churn Rate</p>
                      <p className="text-xl font-black text-emerald-400">0.0%</p>
                    </div>
                    <div className="bg-white/5 p-4 rounded-2xl border border-white/5 text-center">
                      <p className="text-[10px] font-black uppercase text-blue-100/30 mb-1">ARPU</p>
                      <p className="text-xl font-black">${rootStats?.mrr && rootStats?.active_licenses ? (parseFloat(rootStats.mrr) / rootStats.active_licenses).toFixed(2) : '—'}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* School Health View (Root) */}
        {activeTab === 'schools_health' && user.is_system_admin && (
          <div className="animate-in fade-in slide-in-from-bottom-5 duration-500">
            <div className="bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-md">
              <table className="w-full text-left">
                <thead className="bg-white/5 border-b border-white/10">
                  <tr>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Colegio</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Usuarios</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Licencia</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Plan</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">País</th>
                    <th className="p-5 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Tipo</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-white/5">
                  {loading ? (
                    <tr><td colSpan="6" className="p-20 text-center text-blue-100/30">Cargando datos...</td></tr>
                  ) : schoolStats.length > 0 ? schoolStats.map(school => (
                    <tr key={school.id} className="hover:bg-white/5 transition-colors">
                      <td className="p-5">
                        <div className="flex items-center space-x-3">
                          <div className="w-8 h-8 bg-cyan-500/20 text-cyan-400 rounded-lg flex items-center justify-center font-black text-xs">
                            {school.name?.substring(0, 1)}
                          </div>
                          <div>
                            <p className="font-bold text-sm">{school.name}</p>
                            <p className="text-[10px] text-blue-100/30 font-mono">{school.subdomain}</p>
                          </div>
                        </div>
                      </td>
                      <td className="p-5 text-center">
                        <span className="text-2xl font-black">{school.user_count}</span>
                      </td>
                      <td className="p-5 text-center">
                        <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase ${school.license_status === 'active' ? 'bg-emerald-500/20 text-emerald-400' :
                          school.license_status ? 'bg-yellow-500/20 text-yellow-400' :
                            'bg-red-500/10 text-red-400/60'}`}>
                          {school.license_status || 'sin licencia'}
                        </span>
                      </td>
                      <td className="p-5 text-center">
                        <span className="text-blue-100/50 text-xs uppercase font-bold">{school.license_plan || '—'}</span>
                      </td>
                      <td className="p-5 text-center">
                        <span className="text-lg">{school.country_code ? `🌎 ${school.country_code}` : '—'}</span>
                      </td>
                      <td className="p-5 text-center">
                        <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase ${school.is_system_admin ? 'bg-violet-500/20 text-violet-400' : 'bg-slate-500/10 text-slate-400'}`}>
                          {school.is_system_admin ? 'Sistema' : 'Cliente'}
                        </span>
                      </td>
                    </tr>
                  )) : (
                    <tr><td colSpan="6" className="p-20 text-center text-blue-100/20 italic">No hay datos disponibles.</td></tr>
                  )}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* School Detail View (Root) */}
        {activeTab === 'school_detail' && selectedSchool && (
          <div className="space-y-8 animate-in fade-in slide-in-from-bottom-5 duration-500">
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
              {/* Stats Card */}
              <div className="lg:col-span-2 space-y-8">
                <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md">
                  <h3 className="text-xl font-bold mb-6 flex items-center space-x-2">
                    <span>📊</span> <span>Métricas del Colegio</span>
                  </h3>
                  <div className="grid grid-cols-2 gap-4">
                    <div className="bg-white/5 p-6 rounded-2xl border border-white/5">
                      <p className="text-[10px] font-black uppercase tracking-widest text-blue-100/30 mb-1">Usuarios Registrados</p>
                      <p className="text-3xl font-black">{selectedSchool.user_count || '—'}</p>
                    </div>
                    <div className="bg-white/5 p-6 rounded-2xl border border-white/5">
                      <p className="text-[10px] font-black uppercase tracking-widest text-blue-100/30 mb-1">Estado Licencia</p>
                      <span className={`px-3 py-1 rounded-full text-[10px] font-black uppercase w-fit ${selectedSchool.license_status === 'active' ? 'bg-emerald-500/20 text-emerald-400' :
                        selectedSchool.license_status ? 'bg-yellow-500/20 text-yellow-400' :
                          'bg-red-500/10 text-red-400/60'
                        }`}>
                        {selectedSchool.license_status || 'sin licencia'}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md">
                  <h3 className="text-xl font-bold mb-6 flex items-center space-x-2">
                    <span>💳</span> <span>Información de Licencia</span>
                  </h3>
                  <div className="space-y-4">
                    <div className="flex justify-between items-center p-4 bg-white/5 rounded-xl">
                      <span className="text-sm text-blue-100/50">Plan Actual</span>
                      <span className="font-bold uppercase text-xs bg-indigo-500/20 text-indigo-400 px-3 py-1 rounded-full">
                        {selectedSchool.license_plan || 'N/A'}
                      </span>
                    </div>
                    <div className="flex justify-between items-center p-4 bg-white/5 rounded-xl">
                      <span className="text-sm text-blue-100/50">Subdominio Activo</span>
                      <span className="font-bold text-sm text-cyan-400">{selectedSchool.subdomain}.ccb.edu.co</span>
                    </div>
                  </div>
                </div>
              </div>

              {/* Edit Form */}
              <div className="bg-white/5 border border-white/10 p-8 rounded-3xl backdrop-blur-md h-fit">
                <h3 className="text-xl font-bold mb-6 flex items-center space-x-2">
                  <span>⚙️</span> <span>Configuración Escolar</span>
                </h3>
                <SchoolEditForm
                  initialData={selectedSchool}
                  countries={countries}
                  loading={actionLoading}
                  onSubmit={(f) => handleAction('put', `/saas/schools/${selectedSchool.id}`, f, 'Datos del colegio actualizados')}
                />
              </div>
            </div>
          </div>
        )}

        {/* Report Card View */}
        {activeTab === 'report_card' && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-8 animate-in fade-in slide-in-from-bottom-5">
            {loading ? (
              <p className="col-span-full text-center py-20 text-blue-100/30">Calculando promedios...</p>
            ) : reportCard.length > 0 ? reportCard.map((item, idx) => (
              <div key={idx} className="bg-white/5 border border-white/10 p-8 rounded-3xl flex justify-between items-center group hover:bg-white/10 transition-all">
                <div>
                  <h3 className="text-2xl font-black mb-1">{item.course_name}</h3>
                  <div className="flex items-center space-x-4">
                    <span className="text-xs text-blue-100/40 uppercase font-black tracking-widest">Asistencia: {parseFloat(item.attendance_percentage || 0).toFixed(0)}%</span>
                  </div>
                </div>
                <div className={`p-6 rounded-2xl text-center min-w-[100px] border ${parseFloat(item.average_grade) >= 4.0 ? 'bg-cyan-500/10 border-cyan-500/50 text-cyan-400' :
                  parseFloat(item.average_grade) >= 3.0 ? 'bg-yellow-500/10 border-yellow-500/50 text-yellow-400' :
                    'bg-red-500/10 border-red-500/50 text-red-400'
                  }`}>
                  <p className="text-[10px] font-black uppercase mb-1 opacity-60">Promedio</p>
                  <p className="text-3xl font-black">{parseFloat(item.average_grade).toFixed(1)}</p>
                </div>
              </div>
            )) : (
              <p className="col-span-full text-center py-20 text-blue-100/20 italic">Aún no tienes notas o asistencia registrada en este periodo.</p>
            )}
          </div>
        )}

        {/* Dynamic Lists (Courses, Teachers, Students, Members) */}
        {(activeTab === 'courses' || activeTab === 'teachers' || activeTab === 'students' || activeTab === 'course_members') && (
          <div className="animate-in fade-in slide-in-from-bottom-5 duration-500">
            {loading ? (
              <p className="text-center py-20 text-blue-100/30 font-bold uppercase tracking-[0.3em]">Cargando...</p>
            ) : viewingGrades ? (
              <div className="bg-white/5 border border-white/10 rounded-3xl overflow-hidden backdrop-blur-md">
                <table className="w-full text-left">
                  <thead className="bg-white/5 border-b border-white/10">
                    <tr>
                      <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Estudiante</th>
                      <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40">Evaluación</th>
                      <th className="p-6 text-[10px] font-black uppercase tracking-widest text-blue-100/40 text-center">Nota</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {grades.length > 0 ? grades.map(g => (
                      <tr key={g.id} className="hover:bg-white/5 transition-colors">
                        <td className="p-6 font-bold">{g.student_name}</td>
                        <td className="p-6 text-blue-100/60">{g.name}</td>
                        <td className="p-6 text-center">
                          <span className={`px-4 py-2 rounded-xl font-black text-xs ${parseFloat(g.grade) >= 3.5 ? 'bg-cyan-500/10 text-cyan-400' : 'bg-red-500/10 text-red-400'}`}>
                            {parseFloat(g.grade).toFixed(1)}
                          </span>
                        </td>
                      </tr>
                    )) : (
                      <tr>
                        <td colSpan="3" className="p-20 text-center text-blue-100/20 italic">No hay calificaciones registradas para este curso.</td>
                      </tr>
                    )}
                  </tbody>
                </table>
              </div>
            ) : (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {data.length > 0 ? data.map(item => (
                  <div key={item.id || item.user_id} className="bg-white/5 border border-white/10 rounded-3xl p-6 hover:bg-white/10 transition-all group border-l-4 border-l-cyan-500">
                    <div className="flex justify-between items-start mb-4">
                      <h3 className="text-xl font-bold truncate pr-4">{item.name}</h3>
                      {item.grade_level && (
                        <span className="bg-cyan-500/10 text-cyan-400 text-[10px] font-black px-2 py-1 rounded-full uppercase flex-shrink-0">{item.grade_level}</span>
                      )}
                    </div>
                    <p className="text-blue-100/60 text-sm mb-6 min-h-[40px] line-clamp-2">
                      {item.description || item.email || 'Sin información adicional.'}
                    </p>
                    <div className="flex items-center justify-between pt-4 border-t border-white/5">
                      <div className="flex items-center space-x-2">
                        <div className="w-6 h-6 bg-white/10 rounded-full flex items-center justify-center text-[10px]">
                          {activeTab === 'courses' ? '📚' : activeTab === 'teachers' ? '👨‍🏫' : activeTab === 'course_members' ? '🎓' : '👥'}
                        </div>
                        <span className="text-xs text-blue-100/80 font-medium capitalize">
                          {activeTab === 'courses' ? 'Público' : activeTab === 'course_members' ? 'Alumno' : 'Registrado'}
                        </span>
                      </div>
                      <div className="flex space-x-3">
                        {activeTab === 'managed_schools' && (
                          <button
                            onClick={() => {
                              setSelectedItem({ school_id: item.id, school_name: item.name, plan_type: item.license_plan || 'basic', status: item.license_status || 'active' });
                              setModalType('license');
                              setIsModalOpen(true);
                            }}
                            className="text-indigo-400 text-xs font-black uppercase hover:underline"
                          >
                            Licencia
                          </button>
                        )}
                        {activeTab === 'courses' ? (
                          <button
                            onClick={() => {
                              setSelectedItem(item);
                              setActiveTab('course_members');
                              setViewingGrades(false);
                            }}
                            className="text-cyan-400 text-xs font-black uppercase hover:underline"
                          >
                            Ver Alumnos
                          </button>
                        ) : activeTab === 'course_members' ? (
                          <>
                            <button
                              onClick={() => { setSelectedStudent(item); setModalType('attendance'); setIsModalOpen(true); }}
                              className="text-indigo-400 text-xs font-black uppercase hover:underline"
                            >
                              Asistencia
                            </button>
                            <button
                              onClick={() => { setSelectedStudent(item); setModalType('grade'); setIsModalOpen(true); }}
                              className="text-cyan-400 text-xs font-black uppercase hover:underline"
                            >
                              Nota
                            </button>
                          </>
                        ) : (
                          <button className="text-cyan-400 text-xs font-black uppercase hover:underline">Gestionar</button>
                        )}
                      </div>
                    </div>
                  </div>
                )) : (
                  <p className="col-span-full text-center py-20 text-blue-100/30">No se encontraron {activeTab === 'course_members' ? 'alumnos' : activeTab} registrados.</p>
                )}
              </div>
            )}
          </div>
        )}
        {/* Bulk Import View */}
        {activeTab === 'bulk_import' && user.role === 'admin' && (
          <div className="max-w-2xl mx-auto py-10">
            <BulkImport onComplete={() => {
              // Optionally refresh data if needed
            }} />
          </div>
        )}

        {/* Branding View */}
        {activeTab === 'branding' && user.role === 'admin' && (
          <div className="max-w-2xl mx-auto py-10">
            <BrandingConfig
              school={school}
              onUpdate={(updated) => {
                setSchool(updated);
                api.saveSchool(updated);
              }}
            />
          </div>
        )}
      </main>

      {/* Modals */}
      <Modal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} title={getModalTitle()}>
        {modalType === 'enroll' ? (
          <EnrollmentForm loading={actionLoading} onSubmit={f => handleAction('post', '/academic/enrollments', { ...f, course_id: selectedItem.id }, 'Matriculado con éxito')} students={allStudents} />
        ) : modalType === 'grade' ? (
          <GradeForm loading={actionLoading} onSubmit={f => handleAction('post', `/academic/courses/${selectedItem.id}/grades`, { ...f, student_id: selectedStudent.id }, 'Nota guardada')} studentName={selectedStudent?.name} />
        ) : modalType === 'attendance' ? (
          <AttendanceForm loading={actionLoading} onSubmit={f => handleAction('post', `/academic/courses/${selectedItem.id}/attendance`, { ...f, student_id: selectedStudent.id }, 'Asistencia guardada')} studentName={selectedStudent?.name} />
        ) : (
          <>
            {activeTab === 'courses' && <CourseForm loading={actionLoading} onSubmit={f => handleAction('post', '/academic/courses', f, 'Curso creado')} />}
            {activeTab === 'teachers' && <TeacherForm loading={actionLoading} onSubmit={f => handleAction('post', '/academic/teachers', f, 'Profesor registrado')} />}
            {activeTab === 'students' && <StudentForm loading={actionLoading} onSubmit={f => handleAction('post', '/academic/students', f, 'Estudiante inscrito')} />}
            {activeTab === 'managed_schools' && <SchoolForm loading={actionLoading} onSubmit={f => handleAction('post', '/saas/schools', f, 'Colegio registrado en la plataforma')} countries={countries} />}
            {modalType === 'license' && <LicenseForm loading={actionLoading} onSubmit={f => handleAction('post', '/saas/licenses', { ...f, school_id: selectedItem.school_id }, 'Licencia actualizada con éxito')} initialData={selectedItem} />}
          </>
        )}
      </Modal>
    </div>
  );
}

export default App;
