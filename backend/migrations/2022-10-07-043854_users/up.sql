-- Your SQL goes here
CREATE TABLE users (
 id INT(11) PRIMARY KEY AUTO_INCREMENT,
  user_id VARCHAR(60) NOT NULL,
  user_password VARCHAR(60) NOT NULL,
  user_name VARCHAR(60) NOT NULL,
    user_birth VARCHAR(60) NOT NULL,
      user_address VARCHAR(60) NOT NULL,
        user_email VARCHAR(60) NOT NULL,
        
         UNIQUE(user_id),
    UNIQUE(user_email)
        )
