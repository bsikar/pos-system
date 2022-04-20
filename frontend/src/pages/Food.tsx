import "./Item.css";

// get all items from database
const getFood = () => {
  return fetch("http://localhost:8080/api/items/food", { mode: "cors" })
    .then((response) => response.json())
    .then((data) => {
      console.log(data);
      return data;
    });
};

const Food = () => {
  getFood();
  return (
    <div className="Items">
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
      <h2 className="Item">hi</h2>
    </div>
  );
};

export default Food;
