import { Link } from "react-router-dom";
import "./NavigationBar.css";

const NavigationBar = () => {
  return (
    <div className="NavigationBar">
      <li>
        <Link to="/food">Food</Link>
      </li>
      <li>
        <Link to="/drinks">Drinks</Link>
      </li>
      <li>
        <Link to="/other">Other</Link>
      </li>
      <li style={{ float: "right" }}>
        <Link to="/checkout">Cart</Link>
      </li>
    </div>
  );
};

export default NavigationBar;
