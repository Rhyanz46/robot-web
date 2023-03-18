all: 
	cargo build --release
	- rm -r result
	- mkdir result
	- mv target/release/robotweb result/
	- cp chromedriver result/
	cd robot-web/ && yarn build 
	zip -r result/frontend.zip robot-web/dist/

unpack:
	- rm -r result/frontend
	unzip result/frontend.zip -d result/frontend