# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.define :xenial64 do |c|
    c.vm.box = "ubuntu/xenial64"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l ubuntu -c "sh -s -- -y"
      apt-get update
      apt-get install -y gcc libssl-dev pkg-config cmake libdbus-glib-1-dev
      curl https://sh.rustup.rs -sSf | sh -s -- -y
    EOF
  end

  config.vm.define :trusty64 do |c|
    c.vm.box = "ubuntu/trusty64"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l ubuntu -c "sh -s -- -y"
      apt-get update
      apt-get install -y gcc libssl-dev pkg-config cmake libdbus-glib-1-dev
      curl https://sh.rustup.rs -sSf | sh -s -- -y
    EOF
  end

  config.vm.define :centos7 do |c|
    c.vm.box = "centos/7"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l vagrant -c "sh -s -- -y"
      yum install -y gcc openssl-devel cmake dbus-devel
      curl https://sh.rustup.rs -sSf | sh -s -- -y
      cat << EOS > /etc/yum.repos.d/nginx.repo
[nginx]
name=nginx repo
baseurl=http://nginx.org/packages/centos/7/x86_64/
gpgcheck=0
enabled=1
EOS
      yum install -y nginx
      service nginx start
    EOF
    c.vm.network "private_network", ip: "192.168.20.100"
  end

  config.vm.define :centos6 do |c|
    c.vm.box = "centos/6"
    c.vm.provision :shell, inline: <<-EOF
      curl https://sh.rustup.rs -sSf | su -l vagrant -c "sh -s -- -y"
      yum install -y gcc openssl-devel cmake dbus-devel
      curl https://sh.rustup.rs -sSf | sh -s -- -y
      cat << EOS > /etc/yum.repos.d/nginx.repo
[nginx]
name=nginx repo
baseurl=http://nginx.org/packages/centos/6/x86_64/
gpgcheck=0
enabled=1
EOS
      yum install -y nginx
      service nginx start
    EOF
    c.vm.network "private_network", ip: "192.168.20.101"
  end


  config.vm.synced_folder ".", "/vagrant", type: "nfs"
end
