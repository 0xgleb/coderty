import React from "react";
import "./Header.css";

function Header({ account }) {
  return (
    <header>
      <div className="headerContent">
        <h1>Coderty</h1>
        {account && <p>Connected Account: {account}</p>}
      </div>
    </header>
  );
}

export default Header;
