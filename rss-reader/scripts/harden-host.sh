#!/usr/bin/env bash
# First-boot hardening for a Debian/Ubuntu VPS running this app.
# Safe parts (firewall, fail2ban, auto security updates) run by default.
# SSH lock-down is OPT-IN — set HARDEN_SSH=1 and make sure key auth works first.
#
#   sudo ./scripts/harden-host.sh
#   sudo HARDEN_SSH=1 SSH_PORT=22 ./scripts/harden-host.sh
set -euo pipefail

if [[ $EUID -ne 0 ]]; then
	echo "Run as root:  sudo $0" >&2
	exit 1
fi

SSH_PORT="${SSH_PORT:-22}"
HARDEN_SSH="${HARDEN_SSH:-0}"
export DEBIAN_FRONTEND=noninteractive

echo "==> Installing ufw, fail2ban, unattended-upgrades"
apt-get update -y
apt-get install -y ufw fail2ban unattended-upgrades

echo "==> Firewall: allow SSH(${SSH_PORT}), HTTP(80), HTTPS(443); deny other inbound"
ufw allow "${SSH_PORT}/tcp"
ufw allow 80/tcp
ufw allow 443/tcp
ufw default deny incoming
ufw default allow outgoing
ufw --force enable
ufw status verbose

echo "==> Enabling fail2ban (SSH brute-force protection)"
systemctl enable --now fail2ban

echo "==> Enabling automatic security updates"
dpkg-reconfigure -f noninteractive unattended-upgrades || true
systemctl enable --now unattended-upgrades 2>/dev/null || true

if [[ "$HARDEN_SSH" == "1" ]]; then
	echo "==> Hardening SSH: disabling password auth and root login"
	target_home="$(getent passwd "${SUDO_USER:-root}" | cut -d: -f6)"
	if [[ ! -s "${target_home}/.ssh/authorized_keys" ]]; then
		echo "!! No authorized_keys for ${SUDO_USER:-root} at ${target_home}/.ssh/." >&2
		echo "!! Refusing to disable password auth — you would be locked out." >&2
		exit 1
	fi
	conf=/etc/ssh/sshd_config.d/99-hardening.conf
	cat > "$conf" <<-EOF
		PasswordAuthentication no
		PermitRootLogin no
		ChallengeResponseAuthentication no
	EOF
	sshd -t && { systemctl restart ssh 2>/dev/null || systemctl restart sshd; }
	echo "   SSH hardened. Keep this session open and verify a NEW key login works."
fi

cat <<'NOTE'

==> Done.
Note: Docker publishes container ports by writing iptables rules that BYPASS ufw.
This app publishes nothing to the host (Caddy fronts it on 80/443, which ufw allows),
so you are covered. If you ever map a container port to 0.0.0.0, restrict it with a
DOCKER-USER rule or bind it to 127.0.0.1.
NOTE
