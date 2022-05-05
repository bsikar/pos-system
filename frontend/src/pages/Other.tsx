import "./Item.css";
import { useFetch, Item, ItemList } from "../hooks/useFetch";

const Other = () => {
  const items: Item[] = useFetch("/api/items/other");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
};

export default Other;
