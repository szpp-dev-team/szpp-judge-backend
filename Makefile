.PHONY = genall
genall:
	diesel migration revert --all
	diesel migration run
	diesel database setup

.PHONY = db/gen_testdata
db/gen_testdata:
	PGPASSWORD=root psql -h localhost -d szpp-judge -U root -f test/testdata.sql
