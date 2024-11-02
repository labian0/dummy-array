go mod init github.com/labian0/dummy-array
go mod tidy
# python venv creation
cd visualisation
mkdir venv
python3 -m venv venv
venv/bin/pip3 install matplotlib
venv/bin/pip3 install numpy
cd ..

