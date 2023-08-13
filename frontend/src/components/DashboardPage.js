import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import styles from "./Dashboard.module.css";

function DashboardPage() {
  const [address, setAddress] = useState("");
  let navigate = useNavigate();

  const projects = [
    {
      name: "Solidity",
      address: "address1111111111111",
    },
    {
      name: "OpenSourceWallet",
      address: "address1111111111111",
    },
    { name: "Web3", address: "address22222222222222222" },
  ];

  return (
    <div className={styles.wrapper}>
      <div className={styles.col1}>
        {projects.length > 0 ? (
          <>
            <h2>Your Projects</h2>
            <div className="nes-table-responsive">
              <table className="nes-table is-bordered">
                <thead>
                  <tr>
                    <th>Project Name</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {projects.map((project, index) => (
                    <tr key={index}>
                      <td>{project.name}</td>
                      <td>
                        <button
                          className="nes-btn margin-sm-right"
                          onClick={() =>
                            navigate(`/project/${project.address}/manage`)
                          }
                        >
                          Manage
                        </button>
                        <button
                          className="nes-btn"
                          onClick={() =>
                            navigate(`/project/${project.address}/contribute`)
                          }
                        >
                          Contribute
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </>
        ) : (
          <h2>No Projects</h2>
        )}
      </div>
      <div className={styles.col2}>
        <div className={`nes-field ${styles.contribute}`}>
          <input
            type="text"
            id="address_field"
            className={`nes-input ${styles.input}`}
            placeholder="Paste project address"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
          />
          <button
            className="nes-btn"
            htmlFor="name_field"
            onClick={() => navigate(`/project/${address}/contribute`)}
          >
            Contribute
          </button>
        </div>
        <button
          className="nes-btn is-primary"
          onClick={() => navigate("/project/create")}
        >
          + NEW PROJECT
        </button>
      </div>
    </div>
  );
}

export default DashboardPage;
