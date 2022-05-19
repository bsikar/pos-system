import { useFetch, Item, ItemList } from "../components/useFetch";

function Drinks() {
  const items: Item[] = useFetch("/api/items/drinks");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
}

export default Drinks;
