-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: expense
-- ------------------------------

DEFINE TABLE expense TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD amount ON expense TYPE float PERMISSIONS FULL;
DEFINE FIELD date ON expense TYPE datetime VALUE time::now() PERMISSIONS FULL;
DEFINE FIELD description ON expense TYPE string PERMISSIONS FULL;
DEFINE FIELD processed ON expense TYPE bool VALUE false PERMISSIONS FULL;



-- ------------------------------
-- TABLE: had
-- ------------------------------

DEFINE TABLE had TYPE RELATION IN user OUT expense | income ENFORCED SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD in ON had TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON had TYPE record<expense | income> PERMISSIONS FULL;



-- ------------------------------
-- TABLE: income
-- ------------------------------

DEFINE TABLE income TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD amount ON income TYPE float PERMISSIONS FULL;
DEFINE FIELD date ON income TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD description ON income TYPE string PERMISSIONS FULL;
DEFINE FIELD processed ON income TYPE bool DEFAULT false PERMISSIONS FULL;



-- ------------------------------
-- TABLE: user
-- ------------------------------

DEFINE TABLE user TYPE NORMAL SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id, FOR create NONE;

DEFINE FIELD lastname ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD name ON user TYPE string PERMISSIONS FULL;
DEFINE FIELD password ON user TYPE string VALUE crypto::bcrypt::generate(password) PERMISSIONS FOR select NONE, FOR create, update FULL;
DEFINE FIELD username ON user TYPE string PERMISSIONS FULL;

DEFINE INDEX index_username ON user FIELDS username UNIQUE;


