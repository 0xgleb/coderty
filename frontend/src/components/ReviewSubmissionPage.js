import React from "react";
import { useNavigate, useParams } from "react-router-dom";
import styles from "./ReviewSubmission.module.css";

function ReviewSubmissionPage() {
  let navigate = useNavigate();
  const { address, submissionId } = useParams();

  // TODO Web3
  const getSubmission = async () => {
    // get submission from chain using address
  };

  const submission = {
    name: "Name",
    contributor: "contributor",
    link: "link",
    status: "Pending",
    votes: "9/100",
  };

  // TODO Web3
  const getUserData = async () => {
    // get account by connected wallet
    // get user data from chain using address
  };

  const user = {
    votingPower: "33%",
  };

  // TODO Web3
  function handleVote() {}

  return (
    <div className={styles.center}>
      <div className={styles.wrapper}>
        <h1>/{submission.name}</h1>
        <br></br>
        <div className="nes-table-responsive">
          <table className="nes-table is-bordered">
            <thead>
              <tr>
                <th>Submit Name</th>
                <th>Contributor</th>
                <th>Status</th>
                <th>Votes</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>{submission.name}</td>
                <td>{submission.contributor}</td>
                <td>{submission.status}</td>
                <td>{submission.votes}</td>
              </tr>
            </tbody>
          </table>
        </div>
        <br></br>
        <div className={styles.cols}>
          <div className="nes-container margin-sm-right">
            <a href={submission.link}>{submission.link}</a>
            <p>Your voting power: {user.votingPower}</p>
          </div>
          <div className="nes-container is-dark with-title ">
            <p className="title">Review</p>
            <button
              className="nes-btn is-success margin-sm-right"
              onClick={handleVote("approve")}
            >
              Approve
            </button>
            <button className="nes-btn is-error" onClick={handleVote("reject")}>
              Reject
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default ReviewSubmissionPage;
