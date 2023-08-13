import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import styles from "./Dashboard.module.css";

function DashboardPage() {
  const [address, setAddress] = useState("");
  let navigate = useNavigate();

  const shortenString = (text) =>
    text.length > 20 ? text.slice(0, 10) + ".." + text.slice(-10) : text;

  const projects = [
    {
      name: "Solidity",
      link: "address1111111111111",
    },
    { name: "Web3", link: "address22222222222222222" },
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
                    <th>Link</th>
                  </tr>
                </thead>
                <tbody>
                  {projects.map((project, index) => (
                    <tr key={index}>
                      <td>{project.name}</td>
                      <td className={styles.hoverBlue}>
                        {" "}
                        {shortenString(project.link)}
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
          <button
            className="nes-btn"
            htmlFor="name_field"
            onClick={() => navigate(`/project/${address}/contribute`)}
          >
            Contribute
          </button>
          <input
            type="text"
            id="address_field"
            className={`nes-input ${styles.input}`}
            placeholder="Paste project address"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
          />
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
