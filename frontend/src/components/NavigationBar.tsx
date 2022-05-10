import { Link } from "react-router-dom";
import "./NavigationBar.scss";

const NavigationBar = () => {
  return (
    <div className="NavigationBar" id="navigation-bar">
      <Link to="/food">Food</Link>
      <Link to="/drinks">Drinks</Link>
      <Link to="/other">Other</Link>
      <Link to="/checkout">Cart</Link>
      <a
        className="icon"
        onClick={() => {
          document
            .getElementById("navigation-bar")
            ?.classList.toggle("responsive");
        }}
      >
        <i className="fa fa-bars"></i>
      </a>
    </div>
  );
};

export default NavigationBar;
