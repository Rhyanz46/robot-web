build: 
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
	echo "[Unit]\nDescription=Robot Web Backend \nWants=network.target\nAfter=syslog.target network-online.target\n\n[Service]\nType=simple\nExecStart=/home/rhyanz46/robot-web/result/robotweb\nRestart=on-failure\nRestartSec=10\nKillMode=process\nStandardOutput=append:/var/log/backend/out.log\nStandardError=append:/var/log/backend/error.log\n\n[Install]\nWantedBy=multi-user.target" > result/backend.service
	sudo mv result/backend.service /etc/systemd/system/backend.service
	sudo chmod 640 /etc/systemd/system/backend.service
	- systemctl status backend.service
	sudo systemctl daemon-reload
	sudo systemctl enable backend
	sudo systemctl start backend
	sudo systemctl status backend.service

remove-setting:
	sudo systemctl stop backend.service
	sudo systemctl disable backend.service
	sudo rm /etc/systemd/system/backend.service
	sudo systemctl daemon-reload

restart:
	sudo systemctl restart backend.service

see-log:
	journalctl -u backend.service | tail -n 2