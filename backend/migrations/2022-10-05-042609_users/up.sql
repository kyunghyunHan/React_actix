-- Your SQL goes here
CREATE TABLE users (
  id INT(11) PRIMARY KEY AUTO_INCREMENT,
  username  VARCHAR(60) NOT NULL,
  email VARCHAR(60) NOT NULL,
  password  VARCHAR(60) NOT NULL,
  login_session VARCHAR(60) NOT NULL
)