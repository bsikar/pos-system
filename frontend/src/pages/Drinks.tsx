import "./Item.css";
import { useFetch, Item, ItemList } from "../hooks/useFetch";

const Drinks = () => {
  const items: Item[] = useFetch("/api/items/drinks");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
};

export default Drinks;
