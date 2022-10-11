import "./App.css";
import React, { useState } from "react";
import axios from "axios";

function App() {
  const [userName, setUserName] = useState("");
  const [Id, setId] = useState("");
  const [password, setPassword] = useState("");
  const [address, setAddress] = useState("");
  const [emails, setEmails] = useState("");
  const [birth,setBirth]=useState("");



  //create
  const submitHandler = async (e) => {
    e.preventDefault();

    let body2 = [{
      user_id: Id,
      user_password: password,
      user_name: userName,
      user_birth: birth,
      user_address: address,
      user_email: emails,

     
    }];  
    
   
 for (let i =0;i<body2.length;i++){
  await axios
  .post("http://localhost:8000/crate",body2[i])
  .then((res) => console.log(res))
  .catch((err) => {
    console.log(err);
  });
 }
  };
//read
  const read = async () => {
    await axios
      .get("http://localhost:8000/read")
      .then((res) => {
        console.log(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  //update
  const update = async () => {
    await axios
      .post("http://localhost:8000/update",body2)
      .then((res) => {
        console.log(res.data);
      })
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
    let body2 = {
      first_name: "userName",
        last_name: "1",
        email: "2",
     
    };
  return (
    <div className="App">
      <form onSubmit={submitHandler}>
        이름 : <input type="text" value={Id} onChange={(e)=>{setId(e.target.value)}} />
        이름 :  <input type="text" value={password} onChange={(e)=>{setPassword(e.target.value)}} />
        이름 :   <input type="text" value={userName} onChange={(e)=>{setUserName(e.target.value)}} />
        이름 :  <input type="text" value={birth} onChange={(e)=>{setBirth(e.target.value)}} />
        이름 :  <input type="text" value={address} onChange={(e)=>{setAddress(e.target.value)}} />
        이름 :  <input type="text" value={emails} onChange={(e)=>{setEmails(e.target.value)}} />
        <button type="submit">전송(create)</button>
      </form>
      <button onClick={read}>read</button>
      <button onClick={update}>update</button>
      <button onClick={delete_test}>delete</button>
      
  
    </div>
  );
}

export default App;