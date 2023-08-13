import React from "react";
import { useNavigate, useParams } from "react-router-dom";
import styles from "./ManageProject.module.css";

function ManageProjectPage() {
  let navigate = useNavigate();
  const { address } = useParams();

  // TODO Web3
  const getProject = async () => {
    // get project from chain using address
  };

  const project = {
    name: "Web3",
    address: "address",
    gitLink: "gitLink",
    submissions: [
      {
        name: "Name",
        contributor: "contributor",
        link: "link",
        status: "Pending",
        votes: "9/100",
      },
      {
        name: "Name",
        contributor: "contributor",
        link: "link",
        status: "Pending",
        votes: "9/100",
      },
      {
        name: "Name",
        contributor: "contributor",
        link: "link",
        status: "Pending",
        votes: "9/100",
      },
      {
        name: "Name",
        contributor: "contributor",
        link: "link",
        status: "Pending",
        votes: "9/100",
      },
    ],
  };

  return (
    <div className={styles.center}>
      <div className={styles.wrapper}>
        <h1>/{project.name}/ submissions</h1>
        <div className={styles.row}>
          <div></div>
          <div>
            <p>Git backup: {project.gitLink}</p>
            <button className="nes-btn">Change Git backup link</button>
          </div>
        </div>
        <br></br>
        <br></br>
        <div className="nes-table-responsive">
          <table className="nes-table is-bordered">
            <thead>
              <tr>
                <th>Submit Name</th>
                <th>Contributor</th>
                <th>IPFS link</th>
                <th>Status</th>
                <th>Votes</th>
                <th>Review</th>
              </tr>
            </thead>
            <tbody>
              {project.submissions.map((submission, index) => (
                <tr key={index}>
                  <td>{submission.name}</td>
                  <td>{submission.contributor}</td>
                  <td>{submission.link}</td>
                  <td>{submission.status}</td>
                  <td>{submission.votes}</td>
                  <td>
                    <button
                      className="nes-btn"
                      onClick={() =>
                        navigate(
                          `/project/${project.address}/submission/${submission.name}`
                        )
                      }
                    >
                      Review
                    </button>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}

export default ManageProjectPage;
