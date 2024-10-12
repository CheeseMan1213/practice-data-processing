CREATE TABLE IF NOT EXISTS users (
  email       VARCHAR(255) PRIMARY KEY,
  password    VARCHAR(64) NOT NULL,
  deleted_at  TIMESTAMPTZ DEFAULT NULL,
  token       TEXT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS quadratic_formula (
  email            VARCHAR(255) PRIMARY KEY,
  a                FLOAT NOT NULL,
  b                FLOAT NOT NULL,
  c                FLOAT NOT NULL,
  x1               FLOAT DEFAULT NULL,
  x2               FLOAT DEFAULT NULL,
  CONSTRAINT fk_quadratic_formula FOREIGN KEY (email) REFERENCES users(email)
);

INSERT INTO users (email, password) VALUES ('testuser@gmail.com', '$2b$12$x3hs5oMgjHdcV1GUEElfsO19JtS6.ixJAX9Cj62GyhpdPAIW25sky');

INSERT INTO quadratic_formula (email, a, b, c) VALUES ('testuser@gmail.com', 1, 5, 6);
