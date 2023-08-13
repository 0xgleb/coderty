import React from "react";
import { useNavigate, useLocation } from "react-router-dom";
import styles from "./Header.module.css";
import logo from "../images/codertyWhite.png";

function Header({ account }) {
  let navigate = useNavigate();
  const location = useLocation();
  console.log(location.pathname);

  return (
    <div>
      <header>
        <div className={styles.content}>
          <div className={styles.row}>
            <img src={logo} width="60" height="60"></img>
            <h1 className={styles.name}>Coderty</h1>
          </div>
          {account && <p className={styles.account}>{account}</p>}
        </div>
      </header>
      {location.pathname !== "/" && location.pathname !== "/dashboard" && (
        <div className={styles.back} onClick={() => navigate(-1)}>
          {"<--"}
        </div>
      )}
    </div>
  );
}

export default Header;
