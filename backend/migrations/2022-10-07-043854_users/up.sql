-- Your SQL goes here
CREATE TABLE users (
 id INT(11) PRIMARY KEY AUTO_INCREMENT,
  user_id VARCHAR(60) NOT NULL,
  user_pw VARCHAR(200) NOT NULL,
  user_name VARCHAR(60) NOT NULL,
    user_phone VARCHAR(60) NOT NULL,
    
        
         UNIQUE(user_id)

        )
