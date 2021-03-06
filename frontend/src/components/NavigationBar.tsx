import { Link } from "react-router-dom";
import "../sass/NavigationBar.scss";

function NavigationBar() {
  return (
    <div className="navbar">
      <div className="nav-container">
        <input className="checkbox" type="checkbox" id="nav-checkbox" />
        <div className="hamburger-lines">
          <span className="line line1" />
          <span className="line line2" />
          <span className="line line3" />
        </div>
        <div className="menu-items">
          <Link
            to="/food"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox",
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Food &nbsp;
            <i className="fa fa-cutlery" aria-hidden="true" />
          </Link>
          <Link
            to="/drinks"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox",
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Drinks &nbsp;
            <i className="fa fa-coffee" aria-hidden="true" />
          </Link>
          <Link
            to="/other"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox",
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Other &nbsp;
            <i className="fa fa-bullseye" aria-hidden="true" />
          </Link>
          <Link
            to="/checkout"
            onClick={() => {
              const checkBox = document.getElementById(
                "nav-checkbox",
              )! as HTMLInputElement;
              checkBox.checked = false;
            }}
          >
            Cart &nbsp;
            <i className="fa fa-shopping-cart" aria-hidden="true" />
          </Link>
        </div>
      </div>
    </div>
  );
}

export default NavigationBar;
