import React from 'react';
import styles from '../styles/user.module.css'; // Correct the import path if necessary and ensure the file is named as a module

export default function User() {
  return (
    <div className={styles.userPage}>
      <h1>Welcome to the User Page</h1>
      <p>Please verify your account and upload the necessary documents.</p>
      <button className={styles.verifyBtn} onClick={() => alert("Verification process started.")}>Verify Account</button>
      <input
        className={styles.fileInput}
        type="file"
        onChange={(e) => {
          const file = e.target.files ? e.target.files[0] : null;
          if (!file) {
            alert("No file selected.");
            return;
          }
          // Placeholder for file upload logic
          alert(`File ${file.name} uploaded successfully.`);
        }}
      />
    </div>
  );
}