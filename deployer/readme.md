

cargo watch -- python ./deployer/setup.py

// install paramiko 

pip install paramiko

// restart ssh server 

sudo service ssh --full-restart

//disable root login 

sudo vim /etc/ssh/sshd_config

//kill process

sudo ps

sudo kill -9 pid

//open port in ufw

sudo ufw allow 80/tcp 80,443,587,2525