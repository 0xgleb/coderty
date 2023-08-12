import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

import Header from "./Header";
import "./AppWrapper.css";

function AppWrapper({ children }) {
  const [connected, setConnected] = useState(false);
  const [account, setAccount] = useState("");
  let navigate = useNavigate();

  useEffect(() => {
    const checkWalletConnection = async () => {
      if (window.ethereum) {
        try {
          const accounts = await window.ethereum.request({
            method: "eth_accounts",
          });

          if (accounts.length > 0) {
            setConnected(true);
            setAccount(accounts[0]);
          }
        } catch (error) {
          console.error("Error checking wallet connection:", error);
        }
      } else {
        console.error("MetaMask not found");
        navigate("/");
      }
    };

    checkWalletConnection();
  }, [navigate]);

  return (
    <div className="wrapper">
      <Header account={connected ? account : ""} />
      <div className="childrenWrapper">{children}</div>
    </div>
  );
}

export default AppWrapper;
