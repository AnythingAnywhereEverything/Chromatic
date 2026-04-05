import React from 'react';
import { ping } from '../api/ping';
import style from '../styles/home.module.scss';

const Home: React.FC = () => {
  const [message, setMessage] = React.useState('Pinging...');

  React.useEffect(() => {
    async function fetchPing() {
      try {
        const res = await ping();
        setMessage(res);
      } catch (err) {
        console.error(err);
        setMessage('API failed');
      }
    }
    fetchPing();
  }, []);

  return (
    <div className={style.homeContainer}>
      <header>
        <h1>Welcome to Next.js Absolute Cinema Testing!</h1>
        <p>Ping response: {message}</p>
      </header>

      <main>
        <p>
          This is a template homepage example
        </p>
      </main>

      <footer>
        <small>Footer!</small>
      </footer>
    </div>
  );
};

export default Home;
