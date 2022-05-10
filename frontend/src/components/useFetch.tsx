import axios from "axios";
import { useEffect, useState } from "react";
import "./ItemGrid.scss";

export interface Item {
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

export const ItemList = (props: { items: Item[] }) => {
  return (
    <div className="Items">
      {props.items.map((item: Item, id: number) => (
        <button className="Item" key={id}>
          <div>{item.name.replace(/(\r\n|\n|\r)/gm, "")}</div>
        </button>
      ))}
    </div>
  );
};
