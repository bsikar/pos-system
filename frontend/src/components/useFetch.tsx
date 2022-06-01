import axios from "axios";
import { Key, useEffect, useState } from "react";
import "../sass/ItemGrid.scss";

export interface Item {
  id: Key | null | undefined;
  item: any;
  name: string;
  price: number;
  tax: number;
  type: string;
}

export function useFetch(url: string): Item[] {
  const [data, setData] = useState([]);
  const [, setLoading] = useState(false);
  const [, setError] = useState("");

  useEffect(() => {
    setLoading(true);
    axios
      .get(url)
      .then((response) => {
        setData(response.data);
      })
      .catch((err) => {
        setError(err);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [url]);

  return data;
}

export function ItemList(props: { items: Item[] }) {
  const { items } = props;
  return (
    <div className="Items">
      {items.map((thing) => (
        <button className="Item" type="submit" key={thing.id}>
          <div>{thing.name.replace(/(\r\n|\n|\r)/gm, "")}</div>
        </button>
      ))}
    </div>
  );
}
