import React, { useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import styles from "./CreateProject.module.css";

function ContributeToProjectPage() {
  // let navigate = useNavigate();
  const { address } = useParams();

  const [submissionName, setSubmissionName] = useState("");
  const [ipfsHash, setIpfsHash] = useState("");
  const [gitLink, setGitLink] = useState("");

  // TODO Web3
  const handleContribute = async () => {
    // send data from input fields on chain
    // wait for smart contract to create
    // get projectId
    // navigate("/project/:projectId/manage");
  };

  return (
    <div className={styles.center}>
      <div className={styles.wrapper}>
        <h1>Contribute to /Project Name/</h1>
        <p>{address}</p>
        <input
          type="text"
          className="nes-input"
          placeholder="Submission name"
          value={submissionName}
          onChange={(e) => setSubmissionName(e.target.value)}
        />
        <input
          type="text"
          className="nes-input"
          placeholder="Hash of your IPFS patch"
          value={ipfsHash}
          onChange={(e) => setIpfsHash(e.target.value)}
        />
        <input
          type="text"
          className="nes-input"
          placeholder="Link to your GitHub (optional)"
          value={gitLink}
          onChange={(e) => setGitLink(e.target.value)}
        />
        <button className="nes-btn is-primary" onClick={handleContribute}>
          DEPLOY
        </button>
      </div>
    </div>
  );
}

export default ContributeToProjectPage;
