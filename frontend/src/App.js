import React, { useEffect, useState } from 'react';

function App() {
    const [message, setMessage] = useState('');

    useEffect(() => {
        fetch('http://localhost:8080')
            .then(response => response.json())
            .then(data => setMessage(data.message)) // Access the 'message' field
            .catch(error => console.error(error));
    }, []);

    return (
        <div>
            <h1>Hello from React front-end!</h1>
            <p>Message from the backend: {message}</p>
        </div>
    );
}

export default App;
