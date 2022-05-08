import { useFetch, Item, ItemList } from "../components/useFetch";

const Other = () => {
  const items: Item[] = useFetch("/api/items/other");

  return (
    <div>
      <ItemList items={items} />
    </div>
  );
};

export default Other;
