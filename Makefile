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

setting-driver:
	systemd --version
	echo "[Unit]\nDescription=Robot Web Backend Driver\nWants=network.target\nAfter=syslog.target network-online.target\n\n[Service]\nType=simple\nExecStart=/home/rhyanz46/robot-web/result/chromedriver\nRestart=on-failure\nRestartSec=10\nKillMode=process\n\n[Install]\nWantedBy=multi-user.target" > result/backend-driver.service
	sudo mv result/backend-driver.service /etc/systemd/system/backend-driver.service
	sudo chmod 640 /etc/systemd/system/backend-driver.service
	- systemctl status backend-driver.service
	sudo systemctl daemon-reload
	sudo systemctl enable backend-driver
	sudo systemctl start backend-driver
	sudo systemctl status backend-driver.service

remove-driver-setting:
	sudo systemctl stop backend-driver.service
	sudo systemctl disable backend-driver.service
	sudo rm /etc/systemd/system/backend-driver.service
	sudo systemctl daemon-reload

setting:
	systemd --version
	echo "[Unit]\nDescription=Robot Web Backend \nWants=network.target\nAfter=syslog.target network-online.target\n\n[Service]\nType=simple\nExecStart=/home/rhyanz46/robot-web/result/robotweb\nRestart=on-failure\nRestartSec=10\nKillMode=process\n\n[Install]\nWantedBy=multi-user.target" > result/backend.service
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

log-backend:
	journalctl -u backend.service  --follow

log-driver:
	journalctl -u backend.service  --follow