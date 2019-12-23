import React, { useEffect, useState } from 'react';
import ReactDOM from 'react-dom';
import axios from 'axios';

const IndexPage = () => {
  const [info, setInfo] = useState<unknown>(null);
  useEffect(() => {
    axios.get('/api/info').then(({ data }) => setInfo(data));
  }, []);

  return (
    <div>
      <p>Welcome to Kroneum configuration application.</p>
      <p>Device: {JSON.stringify(info)}</p>
      <button onClick={() => axios.get('/api/beep')}>Beep</button>
    </div>
  );
};

ReactDOM.render(<IndexPage />, document.getElementById('root'));
