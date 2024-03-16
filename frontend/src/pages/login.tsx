import React, { useState } from 'react';
import { useRouter } from 'next/router';
import styles from '../styles/Login.module.css';

export default function Login() {
  const [userType, setUserType] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const router = useRouter();

  const handleLogin = async (type) => {
    setIsLoading(true);
    // Simulate an async login operation (e.g., API call)
    setTimeout(() => {
      setUserType(type);
      setIsLoading(false);
      // Navigate to the user or company page after login
      router.push(`/${type}`);
    }, 1000);
  };

  return (
    <div className={styles.loginContainer}>
      <button
        className={styles.loginButton}
        onClick={() => handleLogin('user')}
        disabled={isLoading}
      >
        Login as User
      </button>
      <button
        className={styles.loginButton}
        onClick={() => handleLogin('company')}
        disabled={isLoading}
      >
        Login as Company
      </button>
      {userType && (
        <div className={styles.loginStatus}>
          Logged in as {userType}.
        </div>
      )}
    </div>
  );
}