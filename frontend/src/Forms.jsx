import React, { useState } from 'react';

const InputField = ({ label, type = 'text', value, onChange, placeholder, required = false }) => (
    <div className="space-y-2">
        <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">{label}</label>
        <input
            type={type}
            required={required}
            className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm"
            placeholder={placeholder}
            value={value}
            onChange={(e) => onChange(e.target.value)}
        />
    </div>
);

export const CourseForm = ({ onSubmit, loading }) => {
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [grade, setGrade] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, description, grade_level: grade });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <InputField label="Nombre del Curso" value={name} onChange={setName} placeholder="Ej: Álgebra Avanzada" required />
            <InputField label="Grado / Nivel" value={grade} onChange={setGrade} placeholder="Ej: 10th Grade" />
            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">Descripción</label>
                <textarea
                    className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm min-h-[100px]"
                    placeholder="Breve descripción del curso..."
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                />
            </div>
            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Creando...' : 'Guardar Curso'}
            </button>
        </form>
    );
};

export const TeacherForm = ({ onSubmit, loading }) => {
    const [name, setName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [specialty, setSpecialty] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, email, password, specialty });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <InputField label="Nombre Completo" value={name} onChange={setName} placeholder="Ej: Dra. Elena Nito" required />
            <InputField label="Email Institucional" type="email" value={email} onChange={setEmail} placeholder="elena@ccb.edu.co" required />
            <InputField label="Contraseña Temporal" type="password" value={password} onChange={setPassword} placeholder="••••••••" required />
            <InputField label="Especialidad" value={specialty} onChange={setSpecialty} placeholder="Ej: Física Cuántica" />
            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Registrando...' : 'Registrar Profesor'}
            </button>
        </form>
    );
};

export const StudentForm = ({ onSubmit, loading }) => {
    const [name, setName] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [enrollment, setEnrollment] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, email, password, enrollment_number: enrollment });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <InputField label="Nombre del Estudiante" value={name} onChange={setName} placeholder="Ej: Pepito Perez" required />
            <InputField label="Email de Usuario" type="email" value={email} onChange={setEmail} placeholder="pepito@ccb.edu.co" required />
            <InputField label="Contraseña Temporal" type="password" value={password} onChange={setPassword} placeholder="••••••••" required />
            <InputField label="Número de Matrícula" value={enrollment} onChange={setEnrollment} placeholder="Ej: CCB-2026-001" />
            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Inscribiendo...' : 'Inscribir Estudiante'}
            </button>
        </form>
    );
};

export const EnrollmentForm = ({ onSubmit, loading, students = [] }) => {
    const [studentId, setStudentId] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        if (!studentId) return;
        onSubmit({ student_id: studentId });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">Seleccionar Estudiante</label>
                <div className="relative">
                    <select
                        required
                        className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm appearance-none"
                        value={studentId}
                        onChange={(e) => setStudentId(e.target.value)}
                    >
                        <option value="" className="bg-[#111827]">Selecciona un estudiante...</option>
                        {students.map(s => (
                            <option key={s.id} value={s.id} className="bg-[#111827]">
                                {s.name} ({s.email})
                            </option>
                        ))}
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-white/20">▼</div>
                </div>
            </div>
            <button
                type="submit"
                disabled={loading || !studentId}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em] disabled:opacity-50"
            >
                {loading ? 'Matriculando...' : 'Confirmar Matrícula'}
            </button>
        </form>
    );
};

export const GradeForm = ({ onSubmit, loading, studentName }) => {
    const [name, setName] = useState('');
    const [grade, setGrade] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, grade: parseFloat(grade) });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <div className="bg-white/5 p-4 rounded-2xl border border-white/10 mb-4">
                <p className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 mb-1">Calificando a:</p>
                <p className="text-sm font-bold text-cyan-400">{studentName}</p>
            </div>
            <InputField label="Título de la Evaluación" value={name} onChange={setName} placeholder="Ej: Parcial 1, Tarea Semanal" required />
            <InputField label="Nota (0.0 - 5.0)" type="number" value={grade} onChange={setGrade} placeholder="Ej: 4.5" required />

            <button
                type="submit"
                disabled={loading || !name || !grade}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em] disabled:opacity-50"
            >
                {loading ? 'Guardando...' : 'Registrar Calificación'}
            </button>
        </form>
    );
};

export const AttendanceForm = ({ onSubmit, loading, studentName }) => {
    const [status, setStatus] = useState('present');
    const [notes, setNotes] = useState('');
    const [date, setDate] = useState(new Date().toISOString().split('T')[0]);

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ date, status, notes: notes || null });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <div className="bg-white/5 p-4 rounded-2xl border border-white/10 mb-4">
                <p className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 mb-1">Pasando lista a:</p>
                <p className="text-sm font-bold text-cyan-400">{studentName}</p>
            </div>

            <InputField label="Fecha de Asistencia" type="date" value={date} onChange={setDate} required />

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">Estado de Asistencia</label>
                <div className="grid grid-cols-2 gap-3">
                    {['present', 'absent', 'late', 'justified'].map(s => (
                        <button
                            key={s}
                            type="button"
                            onClick={() => setStatus(s)}
                            className={`p-4 rounded-2xl text-xs font-bold uppercase tracking-widest border transition-all ${status === s
                                ? 'bg-cyan-500/20 border-cyan-500 text-cyan-400 shadow-lg shadow-cyan-500/10'
                                : 'bg-white/5 border-white/10 text-white/40 hover:bg-white/10'
                                }`}
                        >
                            {s === 'present' ? 'Asistió' : s === 'absent' ? 'Faltó' : s === 'late' ? 'Tarde' : 'Excusa'}
                        </button>
                    ))}
                </div>
            </div>

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">Observaciones (Opcional)</label>
                <textarea
                    className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm min-h-[80px]"
                    placeholder="Ej: Llegó 15 min tarde con excusa..."
                    value={notes}
                    onChange={(e) => setNotes(e.target.value)}
                />
            </div>

            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Guardando...' : 'Confirmar Asistencia'}
            </button>
        </form>
    );
};
export const SchoolForm = ({ onSubmit, loading, countries = [] }) => {
    const [name, setName] = useState('');
    const [subdomain, setSubdomain] = useState('');
    const [countryId, setCountryId] = useState('');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, subdomain, country_id: countryId ? parseInt(countryId) : null });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <InputField label="Nombre de la Institución" value={name} onChange={setName} placeholder="Ej: Colegio San Jose" required />
            <InputField label="Subdominio (SaaS)" value={subdomain} onChange={setSubdomain} placeholder="Ej: sanjose (sin .ccb.edu.co)" required />

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">País de Operación</label>
                <div className="relative">
                    <select
                        className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm appearance-none"
                        value={countryId}
                        onChange={(e) => setCountryId(e.target.value)}
                    >
                        <option value="" className="bg-[#111827]">Selecciona un país...</option>
                        {countries.map(c => (
                            <option key={c.id} value={c.id} className="bg-[#111827]">
                                {c.name} ({c.code})
                            </option>
                        ))}
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-white/20">▼</div>
                </div>
            </div>

            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Creando...' : 'Registrar Nuevo Colegio'}
            </button>
        </form>
    );
};
export const SchoolEditForm = ({ onSubmit, loading, countries = [], initialData = {} }) => {
    const [name, setName] = useState(initialData.name || '');
    const [subdomain, setSubdomain] = useState(initialData.subdomain || '');
    const [countryId, setCountryId] = useState(initialData.country_id || '');

    const handleAction = (e) => {
        e.preventDefault();
        onSubmit({ name, subdomain, country_id: countryId ? parseInt(countryId) : null });
    };

    return (
        <form onSubmit={handleAction} className="space-y-5">
            <InputField label="Nombre de la Institución" value={name} onChange={setName} placeholder="Ej: Colegio San Jose" required />
            <InputField label="Subdominio (SaaS)" value={subdomain} onChange={setSubdomain} placeholder="Ej: sanjose" required />

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-[0.2em] text-blue-100/40 ml-1">País de Operación</label>
                <div className="relative">
                    <select
                        className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all text-sm appearance-none"
                        value={countryId}
                        onChange={(e) => setCountryId(e.target.value)}
                    >
                        <option value="" className="bg-[#111827]">Selecciona un país...</option>
                        {countries.map(c => (
                            <option key={c.id} value={c.id} className="bg-[#111827]">
                                {c.name} ({c.code})
                            </option>
                        ))}
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-white/20">▼</div>
                </div>
            </div>

            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Actualizando...' : 'Guardar Cambios'}
            </button>
        </form>
    );
};

export const LicenseForm = ({ loading, onSubmit, initialData = {} }) => {
    const [planType, setPlanType] = useState(initialData.plan_type || 'basic');
    const [status, setStatus] = useState(initialData.status || 'active');
    const [expiryDate, setExpiryDate] = useState(
        initialData.expiry_date ? new Date(initialData.expiry_date).toISOString().split('T')[0] : ''
    );
    const [autoRenew, setAutoRenew] = useState(initialData.auto_renew || false);

    const handleSubmit = (e) => {
        e.preventDefault();
        onSubmit({
            plan_type: planType,
            status: status,
            expiry_date: new Date(expiryDate).toISOString(),
            auto_renew: autoRenew,
        });
    };

    return (
        <form onSubmit={handleSubmit} className="space-y-6">
            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-widest text-blue-100/40 ml-4">Plan</label>
                <div className="relative group">
                    <select
                        className="w-full bg-white/5 border border-white/10 rounded-2xl py-4 px-6 text-sm focus:outline-none focus:border-cyan-500/50 transition-all appearance-none"
                        value={planType}
                        onChange={(e) => setPlanType(e.target.value)}
                    >
                        <option value="basic" className="bg-[#111827]">Basic ($49/mo)</option>
                        <option value="premium" className="bg-[#111827]">Premium ($99/mo)</option>
                        <option value="enterprise" className="bg-[#111827]">Enterprise ($249/mo)</option>
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-white/20">▼</div>
                </div>
            </div>

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-widest text-blue-100/40 ml-4">Estado</label>
                <div className="relative group">
                    <select
                        className="w-full bg-white/5 border border-white/10 rounded-2xl py-4 px-6 text-sm focus:outline-none focus:border-cyan-500/50 transition-all appearance-none"
                        value={status}
                        onChange={(e) => setStatus(e.target.value)}
                    >
                        <option value="active" className="bg-[#111827]">Activa</option>
                        <option value="trial" className="bg-[#111827]">Prueba (Trial)</option>
                        <option value="expired" className="bg-[#111827]">Vencida</option>
                        <option value="suspended" className="bg-[#111827]">Suspendida</option>
                    </select>
                    <div className="absolute right-4 top-1/2 -translate-y-1/2 pointer-events-none text-white/20">▼</div>
                </div>
            </div>

            <div className="space-y-2">
                <label className="text-[10px] font-black uppercase tracking-widest text-blue-100/40 ml-4">Vencimiento</label>
                <input
                    type="date"
                    required
                    className="w-full bg-white/5 border border-white/10 rounded-2xl py-4 px-6 text-sm focus:outline-none focus:border-cyan-500/50 transition-all"
                    value={expiryDate}
                    onChange={(e) => setExpiryDate(e.target.value)}
                />
            </div>

            <div className="flex items-center space-x-3 px-4">
                <input
                    type="checkbox"
                    id="autoRenew"
                    className="w-5 h-5 rounded-lg border-white/10 bg-white/5 accent-cyan-500 focus:ring-0"
                    checked={autoRenew}
                    onChange={(e) => setAutoRenew(e.target.checked)}
                />
                <label htmlFor="autoRenew" className="text-sm font-bold text-blue-100/60 cursor-pointer">Auto-renovar automáticamente</label>
            </div>

            <button
                type="submit"
                disabled={loading}
                className="w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-xs uppercase tracking-[0.2em]"
            >
                {loading ? 'Procesando...' : 'Guardar Licencia'}
            </button>
        </form>
    );
};
