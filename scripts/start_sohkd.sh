pkttyagent -p $(echo $$) | pkexec env XDG_CONFIG_HOME=$XDG_CONFIG_HOME swhkd --config /etc/swhkd/swhkdrc
