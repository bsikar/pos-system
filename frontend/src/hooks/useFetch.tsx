import axios from "axios";
import { useEffect, useState } from "react";

export interface Item {
  name: string;
  price: number;
  tax: number;
  type: string;
}

export function useFetch(url: string) {
  const [data, setData] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

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

  const refetch = () => {
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
  };

  return data;
}

export const ItemList = (props: { items: Item[] }) => {
  return (
    <div className="Items">
      {props.items.map((item: Item, id: number) => (
        <h2 className="Item" key={id}>
          {item.name}
        </h2>
      ))}
    </div>
  );
};
