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
-- TABLE DATA: expense
-- ------------------------------

INSERT [ { amount: 12f, date: d'2026-03-05T18:34:22.256073338Z', description: 'Clasesad', id: expense:1zbfw6fmb2k5hyc1qz4w, processed: false }, { amount: 12f, date: d'2026-03-05T18:34:21.309190985Z', description: 'Clasesad', id: expense:629yem16j6dotvxxs0rl, processed: false }, { amount: 12f, date: d'2026-03-05T18:39:15.686409143Z', description: 'Clasesad', id: expense:9orv69uhfdyxgkajkwhi, processed: false }, { amount: 12f, date: d'2026-03-05T18:37:21.344907035Z', description: 'Clasesad', id: expense:bnhwos2ay1j8yuus64co, processed: false }, { amount: 12f, date: d'2026-03-05T18:37:20.416397914Z', description: 'Clasesad', id: expense:cwia81rsr0am9tylfyy2, processed: false }, { amount: 12f, date: d'2026-03-05T18:38:59.428603129Z', description: 'Clasesad', id: expense:e1kiv5qgep212gf0ov91, processed: false }, { amount: 12f, date: d'2026-03-05T18:37:42.325562816Z', description: 'Clasesad', id: expense:erl9pukl2b302aolj2jm, processed: false }, { amount: 834f, date: d'2026-03-05T18:26:03.676998632Z', description: 'Clasesad', id: expense:geg21kzl4p0vz373jzq7, processed: false }, { amount: 834f, date: d'2026-03-05T18:33:24.349894115Z', description: 'Clasesad', id: expense:h68p65x4yisxk7dfa67l, processed: false }, { amount: 12f, date: d'2026-03-05T18:41:24.634504078Z', description: 'Clasesad', id: expense:kl2n1ve0oanhmwsuirsj, processed: false }, { amount: 834f, date: d'2026-03-05T18:24:45.617718928Z', description: 'Clasesad', id: expense:ky1znye6f7jzpsgiv5xx, processed: false }, { amount: 12f, date: d'2026-03-05T18:45:17.147926172Z', description: 'Clasesad', id: expense:m13u826grztjj6xdy357, processed: false }, { amount: 12f, date: d'2026-03-05T18:34:21.941997574Z', description: 'Clasesad', id: expense:mrfc9j7gt4qhtnh9udh4, processed: false }, { amount: 12f, date: d'2026-03-05T18:41:25.773930417Z', description: 'Clasesad', id: expense:mx3kwockwy7febgizusq, processed: false }, { amount: 12f, date: d'2026-03-05T18:41:24.026346588Z', description: 'Clasesad', id: expense:mxdom32ko6oajb8yfkbw, processed: false }, { amount: 12f, date: d'2026-03-05T18:45:17.718885354Z', description: 'Clasesad', id: expense:pv0hu5aaghw8q8rz1js3, processed: false }, { amount: 12f, date: d'2026-03-05T18:41:25.203191236Z', description: 'Clasesad', id: expense:s34d8u2iz5ztt5k0ziet, processed: false }, { amount: 12f, date: d'2026-03-05T18:34:20.478902251Z', description: 'Clasesad', id: expense:uwv7h3fugidzcavyktdn, processed: false }, { amount: 12f, date: d'2026-03-05T18:33:32.229433131Z', description: 'Clasesad', id: expense:vxeqw8jh18z68ucuxg88, processed: false }, { amount: 12f, date: d'2026-03-05T18:45:18.372032563Z', description: 'Clasesad', id: expense:wltx609yk3go6kg7wr8q, processed: false } ];

-- ------------------------------
-- TABLE: had
-- ------------------------------

DEFINE TABLE had TYPE RELATION IN user OUT expense | income ENFORCED SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD in ON had TYPE record<user> PERMISSIONS FULL;
DEFINE FIELD out ON had TYPE record<expense | income> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: had
-- ------------------------------

INSERT RELATION [ { id: had:1fwgm4vqk210vr7uzy6i, in: user:ih4zk62yst3ltv7tgzl3, out: income:dj3sqjqnotbkjix6zms4 }, { id: had:1isaerchfj22drqast9s, in: user:ih4zk62yst3ltv7tgzl3, out: income:j9q2kko3h88azr0tog94 }, { id: had:1sl3r7ak7lwi20wcb1kb, in: user:ih4zk62yst3ltv7tgzl3, out: expense:kl2n1ve0oanhmwsuirsj }, { id: had:1ug8ix76lb938l4tgb7n, in: user:ih4zk62yst3ltv7tgzl3, out: expense:uwv7h3fugidzcavyktdn }, { id: had:3zjpfiw965cunas3cbfn, in: user:ih4zk62yst3ltv7tgzl3, out: expense:9orv69uhfdyxgkajkwhi }, { id: had:4sjla9yzi47040dieyq2, in: user:ih4zk62yst3ltv7tgzl3, out: income:b5tjkk8vgk8vujxiqlha }, { id: had:57bpeehj0duoabo4ijnq, in: user:ih4zk62yst3ltv7tgzl3, out: expense:e1kiv5qgep212gf0ov91 }, { id: had:6r41y9gbd8qtqyi6drgg, in: user:ih4zk62yst3ltv7tgzl3, out: expense:mrfc9j7gt4qhtnh9udh4 }, { id: had:7ezlgkgn1n82k1kyv2ax, in: user:ih4zk62yst3ltv7tgzl3, out: expense:wltx609yk3go6kg7wr8q }, { id: had:80dpbdx92o7kkkh2z0yv, in: user:ih4zk62yst3ltv7tgzl3, out: income:fdzc83xur5qswuvjmzjw }, { id: had:a0o47tex47bw8zhkiq6t, in: user:ih4zk62yst3ltv7tgzl3, out: expense:629yem16j6dotvxxs0rl }, { id: had:b5opjy6pn3o9941c4pgs, in: user:ih4zk62yst3ltv7tgzl3, out: expense:pv0hu5aaghw8q8rz1js3 }, { id: had:bo872rhx9usy9rzyvn7n, in: user:ih4zk62yst3ltv7tgzl3, out: income:cq0clmp3oxlxksyxdwej }, { id: had:c3xrx8wsw0w5ndcge6rc, in: user:ih4zk62yst3ltv7tgzl3, out: expense:h68p65x4yisxk7dfa67l }, { id: had:dcdrkj1apj9h411ltaar, in: user:ih4zk62yst3ltv7tgzl3, out: income:iubet9i81k21obfaglnv }, { id: had:ehlln905n5npcnaphx4n, in: user:ih4zk62yst3ltv7tgzl3, out: income:hqsfiltzxv0qltjqfytz }, { id: had:fc4035ox3rgz73kvhx19, in: user:ih4zk62yst3ltv7tgzl3, out: expense:erl9pukl2b302aolj2jm }, { id: had:gd18d8gqz4qztzsge2pf, in: user:ih4zk62yst3ltv7tgzl3, out: expense:vxeqw8jh18z68ucuxg88 }, { id: had:ggrgk1tls6zhvxcxdinq, in: user:ih4zk62yst3ltv7tgzl3, out: expense:cwia81rsr0am9tylfyy2 }, { id: had:hey98l4tgpj5swm1gv6w, in: user:ih4zk62yst3ltv7tgzl3, out: income:lzplc39h7as5ekmemril }, { id: had:j9pid9l8tvz2xnvkn0qo, in: user:ih4zk62yst3ltv7tgzl3, out: income:7mjz1sijrundag7aw7t4 }, { id: had:jcsrw0f05va5t8c0yueu, in: user:ih4zk62yst3ltv7tgzl3, out: expense:1zbfw6fmb2k5hyc1qz4w }, { id: had:k4l8o145a2y6xivcwtxq, in: user:ih4zk62yst3ltv7tgzl3, out: income:g82osroaqmy8kbfdj7wx }, { id: had:l06uuno1p7x7b57z0vmf, in: user:ih4zk62yst3ltv7tgzl3, out: expense:m13u826grztjj6xdy357 }, { id: had:lpyzzhp9s667emqr88jj, in: user:ih4zk62yst3ltv7tgzl3, out: expense:mxdom32ko6oajb8yfkbw }, { id: had:m5utco8vcpvnk535q9yr, in: user:ih4zk62yst3ltv7tgzl3, out: income:mfu4097gj5m974o1ww3a }, { id: had:n4gj7nrlhw6zw8et05az, in: user:ih4zk62yst3ltv7tgzl3, out: expense:mx3kwockwy7febgizusq }, { id: had:or97gs42bmb1mna734go, in: user:ih4zk62yst3ltv7tgzl3, out: expense:s34d8u2iz5ztt5k0ziet }, { id: had:pmtfgyw4lbq4kb8hwsxi, in: user:ih4zk62yst3ltv7tgzl3, out: income:goigp8oummm8nn3phcbe }, { id: had:qh807r29a90dzhqj28q2, in: user:ih4zk62yst3ltv7tgzl3, out: income:jl983mp24n14i7kfx8z4 }, { id: had:tera429rhp3sgrhg2bd8, in: user:ih4zk62yst3ltv7tgzl3, out: expense:ky1znye6f7jzpsgiv5xx }, { id: had:uo2giu1et09e0zw0xyrf, in: user:ih4zk62yst3ltv7tgzl3, out: expense:bnhwos2ay1j8yuus64co }, { id: had:uu6vgvt1rjp90rd0j527, in: user:ih4zk62yst3ltv7tgzl3, out: income:rw7vkni19b1mu7ctx4lp }, { id: had:vkh54xod072roxxijxbc, in: user:ih4zk62yst3ltv7tgzl3, out: income:1866ratqphsnv8rtp4n4 }, { id: had:xrfdpiw0vapumhji2zmr, in: user:ih4zk62yst3ltv7tgzl3, out: income:dpglh1ldrr0y7g40mppp }, { id: had:zl3qe2f43xwov7olnui7, in: user:ih4zk62yst3ltv7tgzl3, out: expense:geg21kzl4p0vz373jzq7 } ];

-- ------------------------------
-- TABLE: income
-- ------------------------------

DEFINE TABLE income TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD amount ON income TYPE float PERMISSIONS FULL;
DEFINE FIELD date ON income TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD description ON income TYPE string PERMISSIONS FULL;
DEFINE FIELD processed ON income TYPE bool DEFAULT false PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: income
-- ------------------------------

INSERT [ { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:1866ratqphsnv8rtp4n4, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:7mjz1sijrundag7aw7t4, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:b5tjkk8vgk8vujxiqlha, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:cq0clmp3oxlxksyxdwej, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:dj3sqjqnotbkjix6zms4, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:dpglh1ldrr0y7g40mppp, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:fdzc83xur5qswuvjmzjw, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:g82osroaqmy8kbfdj7wx, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:goigp8oummm8nn3phcbe, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:hqsfiltzxv0qltjqfytz, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:iubet9i81k21obfaglnv, processed: true }, { amount: 234f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:j9q2kko3h88azr0tog94, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:jl983mp24n14i7kfx8z4, processed: true }, { amount: 27f, date: d'2020-12-12T00:00:00Z', description: 'new desc', id: income:lzplc39h7as5ekmemril, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:mfu4097gj5m974o1ww3a, processed: true }, { amount: 834f, date: d'2022-12-11T00:00:00Z', description: 'Clasesad', id: income:rw7vkni19b1mu7ctx4lp, processed: true } ];

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

INSERT [ { id: user:gbusgapi7p5yz6vlr8g5, lastname: 'Torres', name: 'Rober', password: '$2b$12$mTFm4vNyHJ8Jx5NNz85hVO1eiUDJC09AEYcN5HxsCtBl6iCsGiTyu', username: 'holi' }, { id: user:ih4zk62yst3ltv7tgzl3, lastname: 'Torres', name: 'Rober', password: '$2b$12$WDd71k.XZrlYQfNQu5Z1JeGE1QMAIgW8lsDjZ62bbi/HBVil9iamu', username: 'robpi' }, { id: user:ohja7d6vxp195zqacq7h, lastname: 'Torres', name: 'Rober', password: '$2b$12$KHFQ7aee2WaD4hzq0PP05un/ciD0YAG/62katfAA7tI3hbqIt7dD.', username: 'drako' }, { id: user:wbjp3t5cjhf674cyjujh, lastname: 'test', name: 'test', password: '$2b$12$CGIyR3eRMHFQdqUH9KzCeeTH4WMNP6fF3.ZJ5EjYl9rq8vZWIowAa', username: 'test' } ];

