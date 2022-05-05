import "./Item.css";
import { useFetch, Item, ItemList } from "../hooks/useFetch";

const Food = () => {
  const items: Item[] = useFetch("/api/items/food");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
};

export default Food;
