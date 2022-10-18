-- Your SQL goes here
CREATE TABLE resumes (
 id INT(11) PRIMARY KEY AUTO_INCREMENT,
  cv_address VARCHAR(60) NOT NULL,
  cv_email VARCHAR(200) NOT NULL,
  cv_letter VARCHAR(60) NOT NULL,
     cv_edu VARCHAR(60) NOT NULL,
      cv_cert VARCHAR(60) NOT NULL,
       cv_awards VARCHAR(60) NOT NULL,
        cv_project VARCHAR(60) NOT NULL,
cv_user_key INT NOT NULL,

 CONSTRAINT resumes_user
        FOREIGN KEY(cv_user_key)
            REFERENCES users(id)
     
        );
        
CREATE TABLE techs (
id INT(11) PRIMARY KEY AUTO_INCREMENT,
tc_list VARCHAR(60) NOT NULL,
tc_user_key INT NOT NULL,
tc_resume_key INT NOT NULL,
 CONSTRAINT techs_user
        FOREIGN KEY(tc_user_key)
            REFERENCES users(id),
 CONSTRAINT techs_resumes
        FOREIGN KEY(tc_resume_key)
            REFERENCES resumes(id)
        );

        
