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

setting:
	systemd --version
	echo "[Unit]
		Description=Robot Web Backend 

		Wants=network.target
		After=syslog.target network-online.target

		[Service]
		Type=simple
		ExecStart=`pwd`/result/robotweb
		Restart=on-failure
		RestartSec=10
		KillMode=process

		[Install]
		WantedBy=multi-user.target" > result/backend.service
	sudo mv result/backend.service /etc/systemd/system/backend.service
	sudo chmod 640 /etc/systemd/system/backend.service
	systemctl status backend.service
	sudo systemctl daemon-reload
	sudo systemctl enable backend
	sudo systemctl start backend
	sudo systemctl status backend.service