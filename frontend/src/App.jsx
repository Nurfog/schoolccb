import React, { useState, useEffect } from 'react';
import api from './api';
import Login from './Login';
import Modal from './Modal';
import { CourseForm, TeacherForm, StudentForm, EnrollmentForm, GradeForm, AttendanceForm } from './Forms';

const SidebarItem = ({ icon, label, active, onClick }) => (
  <button
    onClick={onClick}
    className={`w-full flex items-center space-x-3 px-4 py-3 rounded-xl transition-all duration-300 ${active
        ? 'bg-cyan-500 text-indigo-900 font-bold shadow-lg shadow-cyan-500/20'
        : 'text-blue-100/70 hover:bg-white/10 hover:text-white'
      }`}
  >
    <span className="text-xl">{icon}</span>
    <span className="text-sm tracking-wide">{label}</span>
  </button>
);

function App() {
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

  const fetchData = async () => {
    if (activeTab === 'dashboard' || activeTab === 'settings') return;

    setLoading(true);
    try {
      if (activeTab === 'report_card') {
        const result = await api.get('/academic/my-report-card');
        setReportCard(result);
      } else {
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

  const getModalTitle = () => {
    if (modalType === 'enroll') return `Matricular en ${selectedItem?.name}`;
    if (modalType === 'grade') return 'Registrar Calificación';
    if (modalType === 'attendance') return 'Pasar Lista';
    if (activeTab === 'courses') return 'Crear Nuevo Curso';
    if (activeTab === 'teachers') return 'Registrar Nuevo Profesor';
    if (activeTab === 'students') return 'Inscribir Nuevo Estudiante';
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
        <div className="flex items-center space-x-3 px-2 cursor-pointer" onClick={() => { setActiveTab('dashboard'); setViewingGrades(false); }}>
          <div className="w-10 h-10 bg-cyan-500 rounded-xl flex items-center justify-center font-black text-indigo-950 text-xl shadow-lg shadow-cyan-500/30">
            C
          </div>
          <span className="text-2xl font-black tracking-tighter">Colegio<span className="text-cyan-400">CCB</span></span>
        </div>

        <nav className="flex-1 space-y-2">
          <SidebarItem icon="📊" label="Dashboard" active={activeTab === 'dashboard' && !viewingGrades} onClick={() => { setActiveTab('dashboard'); setViewingGrades(false); }} />
          {user.role === 'alumno' ? (
            <SidebarItem icon="📋" label="Mi Boletín" active={activeTab === 'report_card'} onClick={() => setActiveTab('report_card')} />
          ) : (
            <>
              <SidebarItem icon="📚" label="Mis Cursos" active={activeTab === 'courses' || activeTab === 'course_members'} onClick={() => { setActiveTab('courses'); setViewingGrades(false); }} />
              <SidebarItem icon="👨‍🏫" label="Profesores" active={activeTab === 'teachers'} onClick={() => { setActiveTab('teachers'); setViewingGrades(false); }} />
              <SidebarItem icon="👥" label="Estudiantes" active={activeTab === 'students'} onClick={() => { setActiveTab('students'); setViewingGrades(false); }} />
            </>
          )}
          <SidebarItem icon="⚙️" label="Configuración" active={activeTab === 'settings'} onClick={() => { setActiveTab('settings'); setViewingGrades(false); }} />
        </nav>

        {activePeriod && (
          <div className="bg-cyan-500/5 border border-cyan-500/20 p-4 rounded-2xl">
            <p className="text-[9px] font-black uppercase tracking-widest text-cyan-400/60 mb-1">Periodo Actual</p>
            <p className="text-xs font-bold truncate">{activePeriod.name}</p>
          </div>
        )}

        <div className="pt-6 border-t border-white/10">
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
              <h2 className="text-4xl font-black tracking-tight capitalize">
                {activeTab === 'dashboard' ? 'Panel de Control' : activeTab === 'report_card' ? 'Mi Reporte Académico' : activeTab === 'course_members' ? `Aula: ${selectedItem?.name}` : activeTab}
              </h2>
            </div>
            <p className="text-blue-100/50 text-sm font-medium italic">
              {activeTab === 'course_members' ? 'Gestión de alumnos, asistencia y calificaciones.' : activeTab === 'report_card' ? 'Resumen global de notas y asistencia por curso.' : 'Sección administrativa del colegio central Bogotá.'}
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

            {(user.role === 'admin' || (user.role === 'profesor' && (activeTab === 'courses' || activeTab === 'course_members'))) && (
              activeTab !== 'dashboard' && activeTab !== 'settings' && !viewingGrades && (
                <button
                  onClick={() => {
                    if (activeTab === 'course_members') {
                      fetchStudentsForEnrollment();
                      setModalType('enroll');
                    } else {
                      setModalType('create');
                    }
                    setIsModalOpen(true);
                  }}
                  className="bg-cyan-500 text-indigo-900 font-bold px-6 py-3 rounded-xl shadow-lg shadow-cyan-500/20 hover:scale-105 transition-all active:scale-95"
                >
                  {activeTab === 'course_members' ? '+ Matricular Alumno' : `+ Nuevo ${activeTab === 'courses' ? 'Curso' : activeTab === 'teachers' ? 'Profesor' : 'Estudiante'}`}
                </button>
              )
            )}
          </div>
        </header>

        {activeTab === 'dashboard' && (
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
          </>
        )}
      </Modal>
    </div>
  );
}

export default App;
