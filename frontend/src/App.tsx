import "./sass/App.scss";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import NavigationBar from "./components/NavigationBar";
import Food from "./pages/Food";
import Drinks from "./pages/Drinks";
import Other from "./pages/Other";
import Checkout from "./pages/Checkout";

// react functional component
function App() {
  return (
    <Router>
      <NavigationBar />
      <Routes>
        <Route path="/food" element={<Food />} />
        <Route path="/drinks" element={<Drinks />} />
        <Route path="/other" element={<Other />} />
        <Route path="/checkout" element={<Checkout />} />
      </Routes>
    </Router>
  );
}

export default App;
