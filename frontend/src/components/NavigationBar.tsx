import { Link } from "react-router-dom";
import "../sass/NavigationBar.scss";

const NavigationBar = () => {
  return (
    <div className="navbar">
      <div className="nav-container">
        <input className="checkbox" type="checkbox" id="nav-checkbox" />
        <div className="hamburger-lines">
          <span className="line line1"></span>
          <span className="line line2"></span>
          <span className="line line3"></span>
        </div>
        <div className="menu-items">
          <Link
            to="/food"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox"
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Food
          </Link>
          <Link
            to="/drinks"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox"
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Drinks
          </Link>
          <Link
            to="/other"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox"
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Other
          </Link>
          <Link
            to="/checkout"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox"
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Cart
          </Link>
        </div>
      </div>
    </div>
  );
};

export default NavigationBar;
