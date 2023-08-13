import React, { useEffect } from "react";
import { useNavigate } from "react-router-dom";

import "./MetaMaskConnectionPage.css";

function MetaMaskConnectionPage() {
  const navigate = useNavigate();

  useEffect(() => {
    const checkWalletConnection = async () => {
      if (window.ethereum) {
        try {
          const accounts = await window.ethereum.request({
            method: "eth_accounts",
          });

          if (accounts.length > 0) {
            navigate("/dashboard"); // Redirect if wallet is already connected
          }
        } catch (error) {
          console.error("Error checking wallet connection:", error);
        }
      } else {
        console.error("MetaMask not found");
      }
    };

    checkWalletConnection();
  }, [navigate]);

  const handleConnectWallet = async () => {
    if (window.ethereum) {
      try {
        await window.ethereum.request({ method: "eth_requestAccounts" });
        navigate("/dashboard"); // Redirect to the dashboard after connecting
      } catch (error) {
        console.error("Error connecting to MetaMask:", error);
      }
    } else {
      console.error("MetaMask not found");
    }
  };

  return (
    <div className="metaMaskWrapper">
      <div className="col">
        <h1>Welcome to Coderty!</h1>
        <p>
          Connect your wallet to start getting rewards and ownership for
          contributions to open source projects
        </p>
        <br></br>
        <br></br>
        <br></br>
        <div className="typewriter">
          <p>read-write-own your own code!</p>
        </div>
      </div>
      <div className="col">
        <button
          className="nes-btn is-primary metaMaskButton"
          onClick={handleConnectWallet}
        >
          Connect to MetaMask
        </button>
      </div>
    </div>
  );
}

export default MetaMaskConnectionPage;
