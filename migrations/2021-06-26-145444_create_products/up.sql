CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    stock FLOAT NOT NULL,
    -- 点をつけるポイントをcentsに指定
    price INTEGER
)
