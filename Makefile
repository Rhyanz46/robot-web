build: 
	cargo build --release
	- rm -r result
	- mkdir result
	- mv target/release/robotweb result/
	cd robot-web/ && yarn build 
	zip -r result/frontend.zip robot-web/dist/

unpack:
	- rm -r result/frontend
	unzip result/frontend.zip -d result/frontend
	- sudo rm -r /var/www/html/robot-web
	- sudo mkdir /var/www/html/robot-web
	sudo cp -r result/frontend/robot-web/dist/* /var/www/html/robot-web

setting-driver:
	systemd --version
	cp chromedriver result/ && echo "chromedriver ada"
	echo "[Unit]\nDescription=Robot Web Backend Driver\nWants=network.target\nAfter=syslog.target network-online.target\n\n[Service]\nType=simple\nUser=`whoami`\nGroup=admin\nExecStart=`pwd`/result/chromedriver\nRestart=on-failure\nRestartSec=10\nKillMode=process\n\n[Install]\nWantedBy=multi-user.target" > result/backend-driver.service
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
	echo "[Unit]\nDescription=Robot Web Backend \nWants=network.target\nAfter=syslog.target network-online.target\n\n[Service]\nType=simple\nUser=`whoami`\nGroup=admin\nWorkingDirectory=`pwd`/result\nExecStart=`pwd`/result/robotweb\nRestart=on-failure\nRestartSec=10\nKillMode=process\n\n[Install]\nWantedBy=multi-user.target" > result/backend.service
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
	sudo systemctl restart backend-driver.service

log-backend:
	journalctl -u backend.service  --follow

log-driver:
	journalctl -u backend-driver.service  --follow