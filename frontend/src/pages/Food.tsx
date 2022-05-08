import { useFetch, Item, ItemList } from "../components/useFetch";

const Food = () => {
  const items: Item[] = useFetch("/api/items/food");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
};

export default Food;
