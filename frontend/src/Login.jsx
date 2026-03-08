import React, { useState } from 'react';
import api from './api';

function Login({ onLogin }) {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState('');

    const handleSubmit = async (e) => {
        e.preventDefault();
        setLoading(true);
        setError('');

        try {
            const data = await api.post('/auth/login', { email, password });
            api.saveToken(data.token);
            api.saveUser(data.user);
            onLogin(data.user);
        } catch (err) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-[#0a0f1e] text-white p-6 font-sans">
            <div className="w-full max-w-md animate-in fade-in slide-in-from-bottom-10 duration-700">
                <div className="text-center mb-10">
                    <div className="inline-flex items-center justify-center w-16 h-16 bg-cyan-500 rounded-2xl mb-6 shadow-2xl shadow-cyan-500/20 transform hover:rotate-12 transition-transform cursor-pointer">
                        <span className="text-3xl font-black text-indigo-950">C</span>
                    </div>
                    <h1 className="text-4xl font-black tracking-tighter mb-2">Colegio<span className="text-cyan-400">CCB</span></h1>
                    <p className="text-blue-100/50 font-medium">Bienvenido al sistema de administración</p>
                </div>

                <form onSubmit={handleSubmit} className="bg-white/5 backdrop-blur-xl border border-white/10 p-10 rounded-[2.5rem] shadow-2xl space-y-6">
                    {error && (
                        <div className="bg-red-500/10 border border-red-500/20 text-red-400 text-sm p-4 rounded-2xl text-center font-bold">
                            {error}
                        </div>
                    )}

                    <div className="space-y-2">
                        <label className="text-xs font-black uppercase tracking-widest text-blue-100/60 ml-2">Email Institucional</label>
                        <input
                            type="email"
                            required
                            className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all"
                            placeholder="admin@ccb.edu.co"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                        />
                    </div>

                    <div className="space-y-2">
                        <label className="text-xs font-black uppercase tracking-widest text-blue-100/60 ml-2">Contraseña</label>
                        <input
                            type="password"
                            required
                            className="w-full bg-white/5 border border-white/10 p-4 rounded-2xl focus:outline-none focus:border-cyan-500/50 focus:bg-white/10 transition-all"
                            placeholder="••••••••"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                        />
                    </div>

                    <button
                        type="submit"
                        disabled={loading}
                        className={`w-full bg-cyan-500 text-indigo-950 font-black py-4 rounded-2xl shadow-xl shadow-cyan-500/20 hover:scale-[1.02] active:scale-95 transition-all text-sm uppercase tracking-widest ${loading ? 'opacity-50 cursor-not-allowed' : ''}`}
                    >
                        {loading ? 'Iniciando Sesión...' : 'Entrar al Panel'}
                    </button>
                </form>

                <p className="text-center mt-8 text-blue-100/30 text-xs font-medium">
                    ¿Olvidaste tu contraseña? Contacta a soporte técnico.
                </p>
            </div>
        </div>
    );
}

export default Login;
