-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY ,
    username VARCHAR(50) NOT NULL  UNIQUE,
    mobile VARCHAR(20) NOT NULL  UNIQUE,
    email  VARCHAR(50) NOT NULL  UNIQUE,
    first_name VARCHAR(50) NOT NULL,
    given_name VARCHAR(50) NOT NULL,
    encrypted_password VARCHAR(50) NOT NULL,
    avatar VARCHAR(512) NOT NULL,
    locked_at TIMESTAMP NOT NULL,
    current_sign_in_at TIMESTAMP  NOT NULL, 
    current_sign_in_ip VARCHAR(30) NOT NULL, 
    last_sign_in_at TIMESTAMP  NOT NULL, 
    last_sign_in_ip VARCHAR(30) NOT NULL, 
    sign_in_count INT NOT NULL
);

INSERT INTO public.users(
	username, mobile, email, first_name, given_name, encrypted_password, avatar,
	locked_at, current_sign_in_at, current_sign_in_ip, last_sign_in_at, last_sign_in_ip, sign_in_count)
	VALUES ('admin', '18585075312', 'admin@vsquad.com', 'gang', 'chen', '123456', '',
			Current_timestamp,Current_timestamp,'127.0.0.1' , Current_timestamp, '127.0.0.1', 0);