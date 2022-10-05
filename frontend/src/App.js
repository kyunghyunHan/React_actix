import "./App.css";
import React, { useState } from "react";
import axios from "axios";

function App() {
  const [userName, setUserName] = useState("");
  const [Data, setData] = useState("");

  const nameHandler = (e) => {
    e.preventDefault();
    setUserName(e.target.value);
  };
  //create
  const submitHandler = async (e) => {
    e.preventDefault();

    let body = {
      first_name: userName,
      last_name: "1",
      email: "2",
     
    };

    const json = JSON.stringify(body);

    await axios
      .post("http://localhost:8000/write_post", json)
      .then((res) => console.log(res))
      .catch((err) => {
        console.log(err);
      });
  };
  //delete
  const delete_test = async () => {
    await axios
      .delete("http://localhost:8000/delete/4", {
        name: userName,
        identity: "1",
        hometown: "2",
        age: 29,
      })
      .then((res) => {
        console.log(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  //read
  const read = async () => {
    await axios
      .get("http://localhost:8000/read", {})
      .then((res) => {
        console.log(res.data);
        setData(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  //update
  const update = async () => {
    await axios
      .post("http://localhost:8000/echo", {})
      .then((res) => {
        console.log(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  return (
    <div className="App">
      <form onSubmit={submitHandler}>
        이름 : <input type="text" value={userName} onChange={nameHandler} />
        <button type="submit">전송(create)</button>
      </form>
      <button onClick={read}>read</button>
      <button onClick={delete_test}>delete</button>
      <button onClick={update}>update</button>
      <div>read:{Data[0]}</div>
    </div>
  );
}

export default App;