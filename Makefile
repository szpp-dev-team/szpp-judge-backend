.PHONY = genall
genall:
	diesel migration revert --all
	diesel migration run
	diesel database setup

.PHONY = db/gen_testdata
db/gen_testdata:
	PGPASSWORD=root psql -h localhost -d szpp-judge -U root -f test/testdata.sql

.PHONY = cloud_sql_proxy
cloud_sql_proxy:
	mkdir -p cloudsql
	cloud_sql_proxy -dir=cloudsql -instances=szpp-judge-3776:asia-northeast1:szpp-judge=tcp:0.0.0.0:1234