go build -buildmode=c-shared -o dummyarray_go.so ./go/cmd/main;
rm dummyarray_go.h;
mv dummyarray_go.so ./shared_libraries/;