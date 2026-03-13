// API URL configurable por variables de entorno
const API_URL = import.meta.env.VITE_API_URL || 
                (import.meta.env.PROD ? 'http://localhost:8080' : 'http://localhost:8080');

const api = {
    get: (endpoint) => request(endpoint, { method: 'GET' }),
    post: (endpoint, body) => request(endpoint, {
        method: 'POST',
        body: JSON.stringify(body)
    }),
    put: (endpoint, body) => request(endpoint, {
        method: 'PUT',
        body: JSON.stringify(body)
    }),
    delete: (endpoint) => request(endpoint, { method: 'DELETE' }),

    saveToken: (token) => localStorage.setItem('token', token),
    saveUser: (user) => localStorage.setItem('user', JSON.stringify(user)),
    saveSchool: (school) => localStorage.setItem('school', JSON.stringify(school)),
    getToken: () => localStorage.getItem('token'),
    getUser: () => JSON.parse(localStorage.getItem('user')),
    getSchool: () => JSON.parse(localStorage.getItem('school')),
    logout: () => {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
        localStorage.removeItem('school');
    }
};

async function request(endpoint, options = {}) {
    const token = api.getToken();
    const headers = {
        'Content-Type': 'application/json',
        ...(token && { 'Authorization': `Bearer ${token}` }),
        ...options.headers,
    };

    const response = await fetch(`${API_URL}${endpoint}`, { ...options, headers });

    if (response.status === 401) {
        api.logout();
        window.location.reload();
        return;
    }

    const data = await response.json();
    if (!response.ok) {
        throw new Error(data.error || 'Algo salió mal');
    }
    return data;
}

export default api;
