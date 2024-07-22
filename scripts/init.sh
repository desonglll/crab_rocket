cd ./modules/schemas/cb_schema
diesel setup
diesel database reset
diesel migration run
cd ../../..
