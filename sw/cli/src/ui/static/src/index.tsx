import React from 'react'
import ReactDOM from 'react-dom'
import axios from 'axios';

const IndexPage = () => {
    return (
        <div>
            <p>Welcome to Kroneum configuration application.</p>
            <button onClick={() => axios.get('/api/beep')}>Beep</button>
        </div>
    );
};

ReactDOM.render(
    IndexPage(),
    document.getElementById('root'),
);