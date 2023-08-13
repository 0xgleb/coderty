import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import styles from "./CreateProject.module.css";

function CreateProjectPage() {
  // let navigate = useNavigate();

  const [projectName, setProjectName] = useState("");
  const [teamMembers, setTeamMembers] = useState("");
  const [ipfsHash, setIpfsHash] = useState("");
  const [gitBackup, setGitBackup] = useState("");

  const handleCreateProject = async () => {
    console.log(projectName, teamMembers, ipfsHash, gitBackup);
    // send data from input fields on chain
    // wait for smart contract to create
    // get projectId
    // navigate("/project/:projectId/manage");
  };

  return (
    <div className={styles.center}>
      <div className={styles.wrapper}>
        <h1>Create Project</h1>
        <input
          type="text"
          className="nes-input"
          placeholder="Project name"
          value={projectName}
          onChange={(e) => setProjectName(e.target.value)}
        />
        <input
          type="text"
          className="nes-input"
          placeholder="Add team members/contributers"
          value={teamMembers}
          onChange={(e) => setTeamMembers(e.target.value)}
        />
        <input
          type="text"
          className="nes-input"
          placeholder="Hash of first IPFS patch"
          value={ipfsHash}
          onChange={(e) => setIpfsHash(e.target.value)}
        />
        <input
          type="text"
          className="nes-input"
          placeholder="Backup Git"
          value={gitBackup}
          onChange={(e) => setGitBackup(e.target.value)}
        />
        <button className="nes-btn is-primary" onClick={handleCreateProject}>
          DEPLOY
        </button>
      </div>
    </div>
  );
}

export default CreateProjectPage;
