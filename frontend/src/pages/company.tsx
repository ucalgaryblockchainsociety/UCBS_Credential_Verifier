import React from 'react';
import styles from '../styles/company.module.css'; 

export default function Company() {
  return (
    <div className={styles.companyPage}>
      <h1>Welcome to the Company Page</h1>
      <p>Please verify your company account and upload the necessary documents.</p>
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