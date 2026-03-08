const API_URL = 'http://localhost:8080';

const api = {
    get: (endpoint) => request(endpoint, { method: 'GET' }),
    post: (endpoint, body) => request(endpoint, {
        method: 'POST',
        body: JSON.stringify(body)
    }),

    saveToken: (token) => localStorage.setItem('token', token),
    saveUser: (user) => localStorage.setItem('user', JSON.stringify(user)),
    getToken: () => localStorage.getItem('token'),
    getUser: () => JSON.parse(localStorage.getItem('user')),
    logout: () => {
        localStorage.removeItem('token');
        localStorage.removeItem('user');
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
