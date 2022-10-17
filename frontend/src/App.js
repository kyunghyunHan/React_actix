import "./App.css";
import React, { useState } from "react";
import axios from "axios";

import { useRef } from 'react';
import { useNavigate } from 'react-router-dom';
import { useCookies } from 'react-cookie'; // useCookies import
function App() {
  const formRef = useRef();
	const [cookies, setCookie, removeCookie] = useCookies(['id']);
  const [userName, setUserName] = useState("");
  const [Id, setId] = useState("");
  const [password, setPassword] = useState("");
 
  const [birth,setBirth]=useState("");


  const navigate = useNavigate();
  const login = (e) => {
		e.preventDefault();
		axios
			.post('http://localhost:8000/login', { // 로그인 요청
				user_id: formRef.current.id.value,
				user_pw: formRef.current.passWord.value,
			})
			.then((res) => {
        console.log(res.data)
					// setCookie('id', res.data);// 쿠키에 토큰 저장
			});
	};
  //create
  const submitHandler = async (e) => {
    e.preventDefault();

    let body2 = [{
      user_id: Id,
      user_pw: password,
      user_name: userName,
      user_phone: birth,
    
    }];  
    
   
 for (let i =0;i<body2.length;i++){
  await axios
  .post("http://localhost:8000/signUp",body2[i])
  .then((res) => console.log(res))
  .catch((err) => {
    console.log(err);
  });
 }
  };
  let body3 = {
    user_id: "d",
    user_pw: "d",


   
  };  
  const read = async () => {
    await axios
      .post("http://localhost:8000/login",body3)
      .then((res) => {
        console.log(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  //update
  let body34 = {
    user_id: "d",



   
  };  
  const logout = async () => {
    await axios
      .get("http://localhost:8000/logout",body34)
      .then((res) => {
        console.log(res.data);
      })
      .catch((err) => {
        console.log(err);
      });
  };
  let body3422 = {
    user_id:"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoa2gzMDQ1IiwiaWF0IjoxNjY1OTk0MDUzMjI4ODI5MDAwLCJleHAiOjE2NjY1OTg4NTMyMjg4MjkwMDB9.qMha9uH9NLAPfn35M-Avq7BJi-PVAmDIkNSTKQmSYg8",
  }; 
    //delete
    const delete_test = async () => {
      await axios
        .get("http://localhost:8000/user/eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJoa2gzMDQ1IiwiaWF0IjoxNjY1OTk0MDUzMjI4ODI5MDAwLCJleHAiOjE2NjY1OTg4NTMyMjg4MjkwMDB9.qMha9uH9NLAPfn35M-Avq7BJi-PVAmDIkNSTKQmSYg8")
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
    const logOut = () => {
      removeCookie('id'); // 쿠키를 삭제
      navigate('/'); // 메인 페이지로 이동
    };
    const token = cookies.id;
  return (
    <div className="App">
      <form ref={formRef} onSubmit={login}>
			<input type="text" name="id" placeholder="id" required />
			<input type="password" name="passWord" placeholder="passWord" required />
			<input type="submit"></input>
		</form>
      <form onSubmit={submitHandler}>
        아이디 : <input type="text" value={Id} onChange={(e)=>{setId(e.target.value)}} />
        비번 :  <input type="text" value={password} onChange={(e)=>{setPassword(e.target.value)}} />
        이름 :   <input type="text" value={userName} onChange={(e)=>{setUserName(e.target.value)}} />
        전화번호:  <input type="text" value={birth} onChange={(e)=>{setBirth(e.target.value)}} />
    
        <button type="submit">전송(create)</button>
      </form>

      <button onClick={read}>read</button>
      <button onClick={logOut}>로그아웃</button>
      <button onClick={logout}>logout</button>
      <button onClick={delete_test}>delete</button>
      
  
    </div>
  );
}

export default App;