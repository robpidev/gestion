-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: user
-- ------------------------------

DEFINE TABLE user TYPE NORMAL SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id, FOR create NONE;

DEFINE FIELD lastname ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD password ON user TYPE string VALUE crypto::bcrypt::generate(password) PERMISSIONS FOR select NONE, FOR create, update FULL;
DEFINE FIELD username ON user TYPE string PERMISSIONS FULL;

DEFINE INDEX index_username ON user FIELDS username UNIQUE;


-- ------------------------------
-- TABLE DATA: user
-- ------------------------------

INSERT [ { id: user:gbusgapi7p5yz6vlr8g5, lastname: 'Torres', name: 'Rober', password: '$2b$12$mTFm4vNyHJ8Jx5NNz85hVO1eiUDJC09AEYcN5HxsCtBl6iCsGiTyu', username: 'holi' }, { id: user:ohja7d6vxp195zqacq7h, lastname: 'Torres', name: 'Rober', password: '$2b$12$KHFQ7aee2WaD4hzq0PP05un/ciD0YAG/62katfAA7tI3hbqIt7dD.', username: 'drako' }, { id: user:wbjp3t5cjhf674cyjujh, lastname: 'test', name: 'test', password: '$2b$12$CGIyR3eRMHFQdqUH9KzCeeTH4WMNP6fF3.ZJ5EjYl9rq8vZWIowAa', username: 'test' } ];

